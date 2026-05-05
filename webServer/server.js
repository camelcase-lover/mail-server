import Fastify from "fastify";
import dotenv from 'dotenv';
import { routes } from "./routes/routes.js";
import net from "net";

dotenv.config();
const web_port = process.env.PORT;
const connection = process.env.MAIL_SERVER || "127.0.0.1:2525";

const [host, portStr] = connection.split(":");
const port = Number(portStr);
const fastify = Fastify({
    logger: true
});

export const smtpClient = () => {
   const client = net.createConnection(
        { host, port},
    () => {
        console.log("SMTP SERVER CONNECTED");
    });

    return client;
}
fastify.register(routes, {prefix: "/"});

async function start() {
    try {
        
        // client();
        console.log("Web server starting")
        console.log(connection);
        await fastify.listen({
            host: "0.0.0.0",
            port: web_port,
        });
    } catch (error) {
        fastify.log.error(error);
        process.exit(1);
    }
}
start();