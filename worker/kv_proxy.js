/*
    Cloudflare workers KV proxy.
    Deploy the code to worker, and set `KEY` env as the key to access.
    Then bind a workspace named `KV` to the worker.
*/

const RESPONSE_HEADERS = {
    "Content-Type": "application/json",
    "Server": "worker-kv-proxy",
};

function key_from_path(pathname) {
    return pathname.trim().substr(1);
}

async function handleRequest(request) {
    // validate key
    if (request.headers.get("Authorization") != KEY) {
        return new Response(null, {
            status: 401,
            headers: RESPONSE_HEADERS
        });
    }
    var key = key_from_path(new URL(request.url).pathname);
    if (key.length == 0) {
        return new Response(null, {
            status: 400,
            headers: RESPONSE_HEADERS
        });
    }

    if (request.method == "GET") {
        var value = await KV.get(key);
        if (value != null) {
            return new Response(JSON.stringify({
                result: value
            }), {
                status: 200,
                headers: RESPONSE_HEADERS
            });
        }
        return new Response(null, {
            status: 404,
            headers: RESPONSE_HEADERS
        });
    } else if (request.method == "PUT") {
        var value = await request.text();
        var ttl = parseInt(request.headers.get("key"), 10);
        var opt = {};
        if (ttl != NaN && ttl >= 60) {
            opt = { expirationTtl: ttl }
        }
        await KV.put(key, value, opt);
        return new Response(JSON.stringify({
            result: 'null'
        }), {
            status: 200,
            headers: RESPONSE_HEADERS
        });
    } else if (request.method == "DELETE") {
        await KV.delete(key);
        return new Response(JSON.stringify({
            result: 'null'
        }), {
            status: 200,
            headers: RESPONSE_HEADERS
        });
    } else {
        return new Response(null, {
            status: 405,
            headers: RESPONSE_HEADERS
        })
    }
}

addEventListener('fetch', event => {
    event.respondWith(handleRequest(event.request))
})