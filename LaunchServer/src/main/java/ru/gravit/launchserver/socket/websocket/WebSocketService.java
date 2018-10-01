package ru.gravit.launchserver.socket.websocket;

import com.google.gson.Gson;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.http.websocketx.TextWebSocketFrame;
import ru.gravit.launchserver.LaunchServer;
import ru.gravit.launchserver.socket.websocket.json.JsonResponseInterface;
import ru.gravit.utils.helper.LogHelper;

public class WebSocketService {
    public WebSocketService(LaunchServer server, Gson gson) {
        this.server = server;
        this.gson = gson;
    }

    private final LaunchServer server;
    private final Gson gson;

    void process(ChannelHandlerContext ctx, TextWebSocketFrame frame)
    {
        String request = frame.text();
        JsonResponseInterface response = gson.fromJson(request, JsonResponseInterface.class);
        try {
            response.execute(this,ctx);
        } catch (Exception e)
        {
            LogHelper.error(e);
            sendObject(ctx,new ExceptionResult(e));
        }
    }
    public void sendObject(ChannelHandlerContext ctx, Object obj)
    {
        ctx.channel().writeAndFlush(new TextWebSocketFrame(gson.toJson(obj)));
    }
    public static class ErrorResult
    {
        public ErrorResult(String error) {
            this.error = error;
            this.type = "requestError";
        }

        public final String error;
        public final String type;
    }
    public static class SuccessResult
    {
        public SuccessResult(String requesttype) {
            this.requesttype = requesttype;
            this.type = "success";
        }

        public final String requesttype;
        public final String type;
    }
    public class ExceptionResult
    {
        public ExceptionResult(Exception e) {
            this.message = e.getMessage();
            this.clazz = e.getClass().getName();
            this.type = "exceptionError";
        }

        public final String message;
        public final String clazz;
        public final String type;
    }
}
