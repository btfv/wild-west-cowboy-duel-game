// WebSocket plugin for macroquad — no sapp_jsutils dependency
// Implements ws_connect, ws_send, ws_recv_len, ws_recv_into
// called directly from Rust via extern "C"

(function () {
    let socket = null;
    const queue = []; // array of Uint8Array
    let failed = 0;

    function readStr(ptr, len) {
        const bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
        return new TextDecoder().decode(bytes);
    }

    function ws_connect(ptr, len) {
        const url = readStr(ptr, len);
        socket = new WebSocket(url);
        socket.binaryType = "arraybuffer";
        socket.onerror = () => { failed = 1; };
        socket.onclose = (e) => { if (e.code !== 1000) failed = 1; };
        socket.onmessage = (e) => {
            const data = typeof e.data === "string"
                ? new TextEncoder().encode(e.data)
                : new Uint8Array(e.data);
            queue.push(data);
        };
    }

    function ws_is_connected() {
        return (socket && socket.readyState === WebSocket.OPEN) ? 1 : 0;
    }

    function ws_failed() {
        return failed;
    }

    function ws_send(ptr, len) {
        if (!socket || socket.readyState !== WebSocket.OPEN) return;
        const bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
        const text = new TextDecoder().decode(bytes);
        socket.send(text);
    }

    function ws_recv_len() {
        if (queue.length === 0) return 0;
        return queue[0].length;
    }

    function ws_recv_into(ptr, len) {
        if (queue.length === 0) return;
        const msg = queue.shift();
        const dst = new Uint8Array(wasm_memory.buffer, ptr, len);
        dst.set(msg);
    }

    function get_query_param(ptr, len, out_ptr, out_len_ptr) {
        const name = readStr(ptr, len);
        const params = new URLSearchParams(window.location.search);
        const value = params.get(name) || "";
        const encoded = new TextEncoder().encode(value);
        const maxLen = new Uint32Array(wasm_memory.buffer, out_len_ptr, 1)[0];
        const writeLen = Math.min(encoded.length, maxLen);
        new Uint8Array(wasm_memory.buffer, out_ptr, writeLen).set(encoded.slice(0, writeLen));
        new Uint32Array(wasm_memory.buffer, out_len_ptr, 1)[0] = writeLen;
    }

    function get_page_origin(out_ptr, out_len_ptr) {
        const origin = window.location.origin;
        const encoded = new TextEncoder().encode(origin);
        const maxLen = new Uint32Array(wasm_memory.buffer, out_len_ptr, 1)[0];
        const writeLen = Math.min(encoded.length, maxLen);
        new Uint8Array(wasm_memory.buffer, out_ptr, writeLen).set(encoded.slice(0, writeLen));
        new Uint32Array(wasm_memory.buffer, out_len_ptr, 1)[0] = writeLen;
    }

    function open_url(ptr, len) {
        window.open(readStr(ptr, len), "_blank");
    }

    function copy_to_clipboard(ptr, len) {
        navigator.clipboard.writeText(readStr(ptr, len)).catch(() => {});
    }

    function is_telegram() {
        const data = window.Telegram && window.Telegram.WebApp && window.Telegram.WebApp.initData;
        return typeof data === "string" && data.length > 0;
    }

    function share_action_label() {
        return is_telegram() ? 1 : 0;
    }

    function share_action(ptr, len) {
        const url = readStr(ptr, len);

        if (is_telegram()) {
            window.Telegram.WebApp.openLink("https://t.me/share/url?url=" + encodeURIComponent(url));
        } else {
            navigator.clipboard.writeText(url).catch(() => {});
        }
    }

    function register_plugin(importObject) {
        importObject.env.ws_connect = ws_connect;
        importObject.env.ws_is_connected = ws_is_connected;
        importObject.env.ws_failed = ws_failed;
        importObject.env.ws_send = ws_send;
        importObject.env.ws_recv_len = ws_recv_len;
        importObject.env.ws_recv_into = ws_recv_into;
        importObject.env.get_query_param = get_query_param;
        importObject.env.get_page_origin = get_page_origin;
        importObject.env.open_url = open_url;
        importObject.env.copy_to_clipboard = copy_to_clipboard;
        importObject.env.share_action_label = share_action_label;
        importObject.env.share_action = share_action;
    }

    miniquad_add_plugin({ register_plugin, on_init: () => {}, version: "1", name: "quad_net" });
})();
