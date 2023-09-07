package pro.gravit.launcher.events.request;

import pro.gravit.launcher.events.RequestEvent;

public class GetAssetUploadInfoRequestEvent extends RequestEvent {
    public String url;
    public AuthRequestEvent.OAuthRequestEvent token;

    public GetAssetUploadInfoRequestEvent() {
    }

    public GetAssetUploadInfoRequestEvent(String url, AuthRequestEvent.OAuthRequestEvent token) {
        this.url = url;
        this.token = token;
    }

    @Override
    public String getType() {
        return "getAssetUploadUrl";
    }
}
