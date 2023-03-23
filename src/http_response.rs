use crate::http_header::HttpHeader;

pub(crate) struct HttpResponse {
    pub(crate) protocol: String,
    pub(crate) status_code: u16,
    pub(crate) header: HttpHeader,
    pub(crate) content: String,
}

impl HttpResponse {
    pub(crate) fn new() -> HttpResponse {
        HttpResponse {
            protocol: "HTTP/1.1".to_string(),
            status_code: 200,
            header: HttpHeader::new(),
            content: "".to_string(),
        }
    }

    pub(crate) fn with_content(mut self, content: String) -> HttpResponse {
        self.content = content;
        self
    }

    pub(crate) fn with_status_code(mut self, status_code: u16) -> HttpResponse {
        self.status_code = status_code;
        self
    }

    pub(crate) fn with_protocol(mut self, protocol: String) -> HttpResponse {
        self.protocol = protocol;
        self
    }

    pub(crate) fn with_header(mut self, header: HttpHeader) -> HttpResponse {
        self.header = header;
        self
    }

    pub(crate) fn to_string(&self) -> String {
        format!("{} {}\r\n{}\r\n\r\n{}", self.protocol, self.status_code.to_string(), self.header.to_string(), self.content)
    }
}

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}