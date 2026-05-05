import net from "net";
import dotenv from "dotenv";
import {smtpClient} from "../server.js"

dotenv.config();

const connection = process.env.MAIL_SERVER;
export const loginController = async (request, reply) => {
    return new Promise((resolve, reject) => {
        const { username, password } = request.body;
        const client = smtpClient();

        let log = "";
        let responded = false;
        let state = "INIT";

        client.on("data", (data) => {
            if (responded) return;

            const text = data.toString();
            log += text;

            switch (state) {
                case "INIT":
                    if (text.startsWith("220")) {
                        client.write("EHLO localhost\r\n");
                        state = "EHLO";
                    }
                    break;

                case "EHLO":
                    if (text.startsWith("250")) {
                        client.write("AUTH LOGIN\r\n");
                        state = "AUTH_USER";
                    }
                    break;

                case "AUTH_USER":
                    if (text.startsWith("334")) {
                        client.write(Buffer.from(username).toString("base64") + "\r\n");
                        state = "AUTH_PASS";
                    }
                    break;

                case "AUTH_PASS":
                    if (text.startsWith("334")) {
                        client.write(Buffer.from(password).toString("base64") + "\r\n");
                        state = "AUTH_DONE";
                    }
                    break;

                case "AUTH_DONE":
                    if (text.startsWith("235")) {
                        responded = true;
                        client.removeAllListeners();
                        client.end();

                        resolve({
                            success: true,
                            message: "Login successful",
                            log,
                        });
                    } 
                    else if (text.startsWith("535") || text.startsWith("500")) {
                        responded = true;
                        client.removeAllListeners();
                        client.end();

                        resolve({
                            success: false,
                            message: "Login failed",
                            log,
                        });
                    }
                    break;
            }
        });

        client.on("error", (err) => {
            if (!responded) {
                responded = true;
                reject(err);
            }
        });
    });
};