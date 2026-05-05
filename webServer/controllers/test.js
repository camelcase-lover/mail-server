export const testController = async(request, reply) => {
    try {
        const message = "The server is up go ahead";
        return reply.send({
            message: message
        })
    } catch (error) {
        console.log("test error", error);
        return reply.code(500).send({
            message: "Internal server error"
        });
    }
}