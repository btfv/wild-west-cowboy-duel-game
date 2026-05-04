import { EventEmitter } from "node:events";
import { Bullet } from "./bullet.js";
import {
	BULLET_SPEED,
	configMsg,
	GAME_H,
	GAME_W,
	now,
	SIZE,
	WIN_SCORE,
} from "./config.js";
import {
	CowObject,
	FastObject,
	OBJ_SIZE,
	randomVx,
	SlowObject,
} from "./objects.js";
import { Player } from "./player.js";

const HEARTBEAT_INTERVAL = 3000;
const TICK_MS = 50;
const CACTUS_COUNT = 14;

const ZONE_TOP = GAME_H * 0.15 + SIZE;
const ZONE_BOTTOM = GAME_H * (1 - 0.15) - SIZE;

function randomPlayerColors() {
	const firstPlayerColors = [55, 50, 45].map(
		(base) => base + Math.floor(Math.random() * 200),
	);

	return [firstPlayerColors, firstPlayerColors.map((color) => 255 - color)];
}

const PLAYER_INIT_POSITIONS = [
	{ x0: 0, y: 0, x1: GAME_W - SIZE },
	{ x0: 0, y: GAME_H - SIZE, x1: GAME_W - SIZE },
];

function rand(min, max) {
	return min + Math.random() * (max - min);
}

function lcg(s) {
	return (Math.imul(s, 2654435761) + 1013904223) >>> 0;
}

function generateCactuses() {
	const seed = (Date.now() * Math.random() * 1000) >>> 0;
	const zone_top = GAME_H * 0.15 + SIZE;
	const zone_bot = GAME_H * 0.85 - SIZE;
	const bucket_w = GAME_W / CACTUS_COUNT;
	const positions = [];

	for (let i = 0; i < CACTUS_COUNT; i++) {
		let h = lcg(seed ^ i);
		const x = ((h % 1000) / 1000) * (bucket_w - 14) + i * bucket_w;
		h = lcg(h);
		const y = ((h % 1000) / 1000) * (zone_bot - zone_top) + zone_top;
		positions.push([x, y]);
	}

	return positions;
}

// Emits: broadcast(msg), send(id, msg)
export class Room extends EventEmitter {
	#status = "pending"; // "pending" | "active" | "ended"
	#players = new Map(); // id -> Player
	#scores = new Map();
	#bullets = [];
	#objects = [];
	#cactuses = [];
	#intervals = [];
	#id;
	#playerColors;

	constructor(id) {
		super();
		this.#id = id;
		this.#playerColors = randomPlayerColors();
	}

	get status() {
		return this.#status;
	}
	get id() {
		return this.#id;
	}

	// --- emit helpers ---

	#broadcast(msg) {
		this.emit("broadcast", msg);
	}
	#send(id, msg) {
		this.emit("send", id, msg);
	}

	#broadcastScores() {
		this.#broadcast({
			type: "scores",
			scores: Object.fromEntries(this.#scores),
		});
	}

	#broadcastCactuses() {
		this.#broadcast({ type: "cactuses", positions: this.#cactuses });
	}

	#broadcastObjects() {
		this.#broadcast({
			type: "objects",
			objects: this.#objects.filter((o) => !o.hit).map((o) => o.toData()),
		});
	}

	#broadcastGameState() {
		this.#broadcastScores();
		this.#broadcastObjects();
		this.#broadcastCactuses();
	}

	#broadcastPlayerStates() {
		for (const [id, p] of this.#players)
			this.#broadcast({ type: "state", id, ...p.toState() });
	}

	// --- game logic ---

	#spawnObjects() {
		const band = (ZONE_BOTTOM - ZONE_TOP) / 3;
		const y = (i) => rand(ZONE_TOP + i * band, ZONE_TOP + (i + 1) * band);
		const x = () => rand(0, GAME_W - OBJ_SIZE);

		const bands = [0, 1, 2].sort(() => Math.random() - 0.5);

		this.#objects = [
			new CowObject(0, x(), y(bands[0])),
			new FastObject(1, x(), y(bands[1]), randomVx()),
			new SlowObject(2, x(), y(bands[2]), randomVx()),
		];
	}

	#resetPlayerPositions(t) {
		[...this.#players.entries()].forEach(([id, player], i) => {
			player.resetToSlot(PLAYER_INIT_POSITIONS[i], t);
			this.#broadcast({ type: "state", id, ...player.toState() });
		});
	}

	reset() {
		this.#bullets = [];
		this.#cactuses = generateCactuses();
		this.#spawnObjects();

		const t = now();

		this.#resetPlayerPositions(t);
		this.#broadcastGameState();
		this.#broadcast({ type: "reset" });
	}

	#onBulletHitObj(b, obj, by, t) {
		b.hit_objects.add(obj.id);
		const msg = obj.modifyBullet(b, by, t);
		if (msg) this.#broadcast(msg);
	}

	#onBulletHitPlayer(b, id, player, by) {
		b.dead = true;
		player.hit = true;

		this.#broadcast({ type: "hit", id, x: b.bx, y: by });

		const score = (this.#scores.get(b.shooter) || 0) + 1;
		this.#scores.set(b.shooter, score);
		this.#broadcastScores();

		if (score >= WIN_SCORE) {
			this.#status = "ended";
			this.#intervals.forEach(clearInterval);
			this.#intervals = [];
			this.#broadcast({
				type: "game_over",
				winner: b.shooter,
				scores: Object.fromEntries(this.#scores),
			});
		} else {
			setTimeout(() => this.reset(), 3000);
		}
	}

	#tickObjects(dt) {
		for (const obj of this.#objects) obj.step(dt);
	}

	#tickBullets(t) {
		for (const b of this.#bullets) {
			if (b.dead) continue;
			const by = b.posY(t);

			if (b.outOfBounds(by)) {
				b.dead = true;
				continue;
			}

			for (const obj of this.#objects) {
				if (
					b.dead ||
					b.hit_objects.has(obj.id) ||
					obj.hit ||
					!obj.hitTest(b.bx, by)
				)
					continue;
				this.#onBulletHitObj(b, obj, by, t);
			}

			for (const [id, player] of this.#players) {
				if (
					b.dead ||
					id === b.shooter ||
					player.hit ||
					!player.hitTest(b.bx, by, t)
				)
					continue;
				this.#onBulletHitPlayer(b, id, player, by);
			}
		}
	}

	#tick() {
		const t = now();
		this.#tickObjects(TICK_MS / 1000);
		this.#tickBullets(t);
		this.#bullets = this.#bullets.filter((b) => !b.dead);
	}

	#heartbeat() {
		this.#broadcastPlayerStates();
		this.#broadcastGameState();
	}

	start() {
		this.#status = "active";
		this.#cactuses = generateCactuses();
		this.#spawnObjects();
		const t = now();
		this.#resetPlayerPositions(t);
		this.#broadcastGameState();
		this.#broadcast({ type: "start" });
		this.#intervals.push(
			setInterval(() => this.#tick(), TICK_MS),
			setInterval(() => this.#heartbeat(), HEARTBEAT_INTERVAL),
		);
	}

	// --- player actions ---

	join(id) {
		if (this.#players.size >= 2) {
			this.#send(id, { type: "full" });
			return false;
		}

		this.#send(id, configMsg());

		const [r, g, b] = this.#playerColors[this.#players.size];
		this.#players.set(id, new Player(r, g, b));

		if (this.#players.size === 1) {
			this.#send(id, { type: "waiting", room_id: this.#id });
		} else {
			this.start();
		}

		return true;
	}

	freeze(id) {
		if (this.#status !== "active") return;
		const player = this.#players.get(id);
		if (!player) return;
		const t = now();
		const bullet = player.applyFreeze(t);
		if (!bullet) return;
		this.#broadcast({ type: "state", id, ...player.toState() });
		this.#bullets.push(
			new Bullet(bullet.bx, bullet.by, bullet.dir, t, id, BULLET_SPEED),
		);
		this.#broadcast({
			type: "bullet",
			x: bullet.bx,
			y: bullet.by,
			dir: bullet.dir,
			spawn_time: t,
		});
	}

	leave(id) {
		this.#players.delete(id);
		this.#scores.delete(id);
		this.#broadcast({ type: "leave", id });
		if (this.#status === "active") {
			this.#status = "pending";
			this.#bullets = [];
			this.#broadcast({ type: "waiting", room_id: this.#id });
		}
	}
}

export class RoomConnectionManager {
	#sockets = new Map();
	#room;

	constructor(room) {
		this.#room = room;
		room.on("broadcast", (msg) => this.#broadcast(msg));
		room.on("send", (id, msg) => this.#sendTo(id, msg));
	}

	#broadcast(msg) {
		const data = JSON.stringify(msg);
		for (const socket of this.#sockets.values()) {
			if (socket.readyState === 1) socket.send(data);
		}
	}

	#sendTo(id, msg) {
		const socket = this.#sockets.get(id);
		if (socket?.readyState === 1) socket.send(JSON.stringify(msg));
	}

	add(id, socket) {
		this.#sockets.set(id, socket);
		const handlers = {
			join: (parsed) => this.#room.join(id, parsed),
			freeze: () => this.#room.freeze(id),
		};
		socket.on("message", (raw) => {
			const parsed = JSON.parse(raw.toString());
			handlers[parsed.type]?.(parsed);
		});
		socket.on("close", () => {
			this.#sockets.delete(id);
			this.#room.leave(id);
		});
	}
}

const rooms = new Map();

export function getOrCreateRoom(roomId) {
	if (!rooms.has(roomId)) {
		const room = new Room(roomId);
		const conn = new RoomConnectionManager(room);
		rooms.set(roomId, { room, conn });
	}
	return rooms.get(roomId);
}
