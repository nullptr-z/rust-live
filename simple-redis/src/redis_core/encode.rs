use std::mem::size_of;

use super::*;

const RESP_FRAME_CAP: usize = 1024;

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            RespFrame::Integer(i) => i.encode(),
            RespFrame::SimpleString(s) => s.encode(),
            RespFrame::Error(e) => e.encode(),
            RespFrame::BulkString(b) => b.encode(),
            RespFrame::NullBulkString(n) => n.encode(),
            RespFrame::Array(a) => a.encode(),
            RespFrame::Null(n) => n.encode(),
            RespFrame::NullArray(n) => n.encode(),
            RespFrame::Boolean(b) => b.encode(),
            RespFrame::Double(d) => d.encode(),
            RespFrame::Map(m) => m.encode(),
            RespFrame::Set(s) => s.encode(),
        }
    }
}

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.0.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.0.len()).as_bytes());
        buf.extend_from_slice(&self.0);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".into()
    }
}

impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".into()
    }
}

impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".into()
    }
}

impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        if self {
            b"#t\r\n".into()
        } else {
            b"$f\r\n".into()
        }
    }
}

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(ret.as_bytes());
        buf
    }
}

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.0.len() * RESP_FRAME_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.0.len()).as_bytes());
        for frame in self.0 {
            buf.extend(frame.encode());
        }
        buf
    }
}

impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let slf = self.0;
        let mut buf: Vec<u8> = Vec::with_capacity(slf.len() * RESP_FRAME_CAP);
        buf.extend_from_slice(format!("%{}\r\n", slf.len()).as_bytes());
        for (key, value) in slf {
            buf.extend(SimpleString(key).encode());
            buf.extend(value.encode());
        }
        buf
    }
}

impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let slf = self.0;
        let mut buf: Vec<u8> = Vec::with_capacity(slf.len() * RESP_FRAME_CAP);
        buf.extend_from_slice(format!("~{}\r\n", slf.len()).as_bytes());
        for value in slf {
            buf.extend(value.encode());
        }
        buf
    }
}
