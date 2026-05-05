import {testController} from "../controllers/test.js";
import { loginController } from "../controllers/login.js";
import dotenv from "dotenv";

dotenv.config();

const allow_orgin = process.env.ALLOWED_ORIGIN;
export async function routes(fastify) {
    
    fastify.addHook("onRequest", async(request, reply) => {
        reply.header("Access-Control-Allow-Origin", allow_orgin);
        reply.header("Access-Control-Allow-Credentials", true);
        reply.header("Access-Control-Allow-Headers", "Authorization, Origin, X-Request-With, Content-Type");
        reply.header("Access-Control-Allow-Methods", "GET, POST, PUT, OPTIONS");
    });

    const preflightPaths = [
        "/login",
        "/test"
    ]

    for(const path of preflightPaths){
        fastify.options(path, async(request, reply) => {
            return reply.code(204).send();
        })
    }
    fastify.get("/test",
        testController
    );
    fastify.post("/login",
        loginController
    );
}
