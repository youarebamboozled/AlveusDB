pub(crate) struct HttpHeader {
    pub(crate) a_im: Option<String>,
    pub(crate) accept: Option<String>,
    pub(crate) accept_charset: Option<String>,
    pub(crate) accept_datetime: Option<String>,
    pub(crate) accept_encoding: Option<String>,
    pub(crate) accept_language: Option<String>,
    pub(crate) accept_control_request_headers: Option<String>,
    pub(crate) accept_control_request_method: Option<String>,
    pub(crate) authorization: Option<String>,
    pub(crate) cache_control: Option<String>,
    pub(crate) connection: Option<String>,
    pub(crate) content_encoding: Option<String>,
    pub(crate) content_length: Option<String>,
    pub(crate) content_md5: Option<String>,
    pub(crate) content_type: Option<String>,
    pub(crate) cookie: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) expect: Option<String>,
    pub(crate) forwarded: Option<String>,
    pub(crate) from: Option<String>,
    pub(crate) host: Option<String>,
    pub(crate) http2_settings: Option<String>,
    pub(crate) if_match: Option<String>,
    pub(crate) if_modified_since: Option<String>,
    pub(crate) if_none_match: Option<String>,
    pub(crate) if_range: Option<String>,
    pub(crate) if_unmodified_since: Option<String>,
    pub(crate) max_forwards: Option<String>,
    pub(crate) origin: Option<String>,
    pub(crate) pragma: Option<String>,
    pub(crate) prefer: Option<String>,
    pub(crate) proxy_authorization: Option<String>,
    pub(crate) range: Option<String>,
    pub(crate) referer: Option<String>,
    pub(crate) te: Option<String>,
    pub(crate) trailer: Option<String>,
    pub(crate) transfer_encoding: Option<String>,
    pub(crate) user_agent: Option<String>,
    pub(crate) upgrade: Option<String>,
    pub(crate) via: Option<String>,
    pub(crate) warning: Option<String>,
}
//TODO: move this to a separate file
impl HttpHeader {
    pub(crate) fn new() -> HttpHeader {
        HttpHeader {
            a_im: None,
            accept: None,
            accept_charset: None,
            accept_datetime: None,
            accept_encoding: None,
            accept_language: None,
            accept_control_request_headers: None,
            accept_control_request_method: None,
            authorization: None,
            cache_control: None,
            connection: None,
            content_encoding: None,
            content_length: None,
            content_md5: None,
            content_type: None,
            cookie: None,
            date: Option::from(crate::utils::get_http_date()),
            expect: None,
            forwarded: None,
            from: None,
            host: None,
            http2_settings: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_range: None,
            if_unmodified_since: None,
            max_forwards: None,
            origin: None,
            pragma: None,
            prefer: None,
            proxy_authorization: None,
            range: None,
            referer: None,
            te: None,
            trailer: None,
            transfer_encoding: None,
            user_agent: None,
            upgrade: None,
            via: None,
            warning: None,
        }
    }

    pub(crate) fn with_content_type(mut self, content_type: String) -> HttpHeader {
        self.content_type = Option::from(content_type);
        self
    }
}

impl std::fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut header = String::new();
        if self.a_im.is_some() {
            header.push_str(format!("A-IM: {}\r\n", self.a_im.as_ref().unwrap()).as_str());
        }
        if self.accept.is_some() {
            header.push_str(format!("Accept: {}\r\n", self.accept.as_ref().unwrap()).as_str());
        }
        if self.accept_charset.is_some() {
            header.push_str(format!("Accept-Charset: {}\r\n", self.accept_charset.as_ref().unwrap()).as_str());
        }
        if self.accept_datetime.is_some() {
            header.push_str(format!("Accept-Datetime: {}\r\n", self.accept_datetime.as_ref().unwrap()).as_str());
        }
        if self.accept_encoding.is_some() {
            header.push_str(format!("Accept-Encoding: {}\r\n", self.accept_encoding.as_ref().unwrap()).as_str());
        }
        if self.accept_language.is_some() {
            header.push_str(format!("Accept-Language: {}\r\n", self.accept_language.as_ref().unwrap()).as_str());
        }
        if self.accept_control_request_headers.is_some() {
            header.push_str(format!("Accept-Control-Request-Headers: {}\r\n", self.accept_control_request_headers.as_ref().unwrap()).as_str());
        }
        if self.accept_control_request_method.is_some() {
            header.push_str(format!("Accept-Control-Request-Method: {}\r\n", self.accept_control_request_method.as_ref().unwrap()).as_str());
        }
        if self.authorization.is_some() {
            header.push_str(format!("Authorization: {}\r\n", self.authorization.as_ref().unwrap()).as_str());
        }
        if self.cache_control.is_some() {
            header.push_str(format!("Cache-Control: {}\r\n", self.cache_control.as_ref().unwrap()).as_str());
        }
        if self.connection.is_some() {
            header.push_str(format!("Connection: {}\r\n", self.connection.as_ref().unwrap()).as_str());
        }
        if self.content_encoding.is_some() {
            header.push_str(format!("Content-Encoding: {}\r\n", self.content_encoding.as_ref().unwrap()).as_str());
        }
        if self.content_length.is_some() {
            header.push_str(format!("Content-Length: {}\r\n", self.content_length.as_ref().unwrap()).as_str());
        }
        if self.content_md5.is_some() {
            header.push_str(format!("Content-MD5: {}\r\n", self.content_md5.as_ref().unwrap()).as_str());
        }
        if self.content_type.is_some() {
            header.push_str(format!("Content-Type: {}\r\n", self.content_type.as_ref().unwrap()).as_str());
        }
        if self.cookie.is_some() {
            header.push_str(format!("Cookie: {}\r\n", self.cookie.as_ref().unwrap()).as_str());
        }
        if self.date.is_some() {
            header.push_str(format!("Date: {}\r\n", self.date.as_ref().unwrap()).as_str());
        }
        if self.expect.is_some() {
            header.push_str(format!("Expect: {}\r\n", self.expect.as_ref().unwrap()).as_str());
        }
        if self.forwarded.is_some() {
            header.push_str(format!("Forwarded: {}\r\n", self.forwarded.as_ref().unwrap()).as_str());
        }
        if self.from.is_some() {
            header.push_str(format!("From: {}\r\n", self.from.as_ref().unwrap()).as_str());
        }
        if self.host.is_some() {
            header.push_str(format!("Host: {}\r\n", self.host.as_ref().unwrap()).as_str());
        }
        if self.http2_settings.is_some() {
            header.push_str(format!("HTTP2-Settings: {}\r\n", self.http2_settings.as_ref().unwrap()).as_str());
        }
        if self.if_match.is_some() {
            header.push_str(format!("If-Match: {}\r\n", self.if_match.as_ref().unwrap()).as_str());
        }
        if self.if_modified_since.is_some() {
            header.push_str(format!("If-Modified-Since: {}\r\n", self.if_modified_since.as_ref().unwrap()).as_str());
        }
        if self.if_none_match.is_some() {
            header.push_str(format!("If-None-Match: {}\r\n", self.if_none_match.as_ref().unwrap()).as_str());
        }
        if self.if_range.is_some() {
            header.push_str(format!("If-Range: {}\r\n", self.if_range.as_ref().unwrap()).as_str());
        }
        if self.if_unmodified_since.is_some() {
            header.push_str(format!("If-Unmodified-Since: {}\r\n", self.if_unmodified_since.as_ref().unwrap()).as_str());
        }
        if self.max_forwards.is_some() {
            header.push_str(format!("Max-Forwards: {}\r\n", self.max_forwards.as_ref().unwrap()).as_str());
        }
        if self.origin.is_some() {
            header.push_str(format!("Origin: {}\r\n", self.origin.as_ref().unwrap()).as_str());
        }
        if self.pragma.is_some() {
            header.push_str(format!("Pragma: {}\r\n", self.pragma.as_ref().unwrap()).as_str());
        }
        if self.prefer.is_some() {
            header.push_str(format!("Prefer: {}\r\n", self.prefer.as_ref().unwrap()).as_str());
        }
        if self.proxy_authorization.is_some() {
            header.push_str(format!("Proxy-Authorization: {}\r\n", self.proxy_authorization.as_ref().unwrap()).as_str());
        }
        if self.range.is_some() {
            header.push_str(format!("Range: {}\r\n", self.range.as_ref().unwrap()).as_str());
        }
        if self.referer.is_some() {
            header.push_str(format!("Referer: {}\r\n", self.referer.as_ref().unwrap()).as_str());
        }
        if self.te.is_some() {
            header.push_str(format!("TE: {}\r\n", self.te.as_ref().unwrap()).as_str());
        }
        if self.trailer.is_some() {
            header.push_str(format!("Trailer: {}\r\n", self.trailer.as_ref().unwrap()).as_str());
        }
        if self.transfer_encoding.is_some() {
            header.push_str(format!("Transfer-Encoding: {}\r\n", self.transfer_encoding.as_ref().unwrap()).as_str());
        }
        if self.upgrade.is_some() {
            header.push_str(format!("Upgrade: {}\r\n", self.upgrade.as_ref().unwrap()).as_str());
        }
        if self.user_agent.is_some() {
            header.push_str(format!("User-Agent: {}\r\n", self.user_agent.as_ref().unwrap()).as_str());
        }
        if self.via.is_some() {
            header.push_str(format!("Via: {}\r\n", self.via.as_ref().unwrap()).as_str());
        }
        if self.warning.is_some() {
            header.push_str(format!("Warning: {}\r\n", self.warning.as_ref().unwrap()).as_str());
        }
        write!(f, "{}", header)
    }
}