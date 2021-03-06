use serde;
use serde_json;
use openssl::ssl;
use std::net::{IpAddr,SocketAddr};
use std::default::Default;
use std::convert::From;

//FIXME: make fixed size depending on hash algorithm
pub type CertFingerprint = Vec<u8>;

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct HttpFront {
    pub app_id:     String,
    pub hostname:   String,
    pub path_begin: String,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct CertificateAndKey {
    pub certificate:       String,
    pub certificate_chain: Vec<String>,
    pub key:               String,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct TlsFront {
    pub app_id:       String,
    pub hostname:     String,
    pub path_begin:   String,
    pub fingerprint:  CertFingerprint,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct TcpFront {
    pub app_id:     String,
    pub ip_address: String,
    pub port:       u16
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct Instance {
    pub app_id:     String,
    pub ip_address: String,
    pub port:       u16
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct HttpProxyConfiguration {
    pub front:           SocketAddr,
    pub front_timeout:   u64,
    pub back_timeout:    u64,
    pub max_connections: usize,
    pub buffer_size:     usize,
    pub public_address:  Option<IpAddr>,
    pub answer_404:      String,
    pub answer_503:      String,
}

impl Default for HttpProxyConfiguration {
  fn default() -> HttpProxyConfiguration {
    HttpProxyConfiguration {
      front:           "127.0.0.1:8080".parse().expect("could not parse address"),
      front_timeout:   5000,
      back_timeout:    5000,
      max_connections: 1000,
      buffer_size:     16384,
      public_address:  None,
      answer_404:      String::from("HTTP/1.1 404 Not Found\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n"),
      answer_503:      String::from("HTTP/1.1 503 your application is in deployment\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n"),
    }
  }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash, Serialize, Deserialize)]
pub struct TlsProxyConfiguration {
    pub front:                     SocketAddr,
    pub front_timeout:             u64,
    pub back_timeout:              u64,
    pub max_connections:           usize,
    pub buffer_size:               usize,
    pub public_address:            Option<IpAddr>,
    pub answer_404:                String,
    pub answer_503:                String,
    pub options:                   u64,
    pub cipher_list:               String,
    pub default_name:              Option<String>,
    pub default_app_id:            Option<String>,
    pub default_certificate:       Option<Vec<u8>>,
    pub default_key:               Option<Vec<u8>>,
    pub default_certificate_chain: Option<String>,
}

impl Default for TlsProxyConfiguration {
  fn default() -> TlsProxyConfiguration {
    TlsProxyConfiguration {
      front:           "127.0.0.1:8443".parse().expect("could not parse address"),
      front_timeout:   5000,
      back_timeout:    5000,
      max_connections: 1000,
      buffer_size:     16384,
      public_address:  None,
      answer_404:      String::from("HTTP/1.1 404 Not Found\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n"),
      answer_503:      String::from("HTTP/1.1 503 your application is in deployment\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n"),
      cipher_list:     String::from(
        "ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:\
        ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:\
        ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:\
        DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384:\
        ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:\
        ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES256-SHA384:\
        ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-SHA384:\
        ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:\
        DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:DHE-RSA-AES256-SHA256:\
        DHE-RSA-AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:\
        ECDHE-RSA-DES-CBC3-SHA:EDH-RSA-DES-CBC3-SHA:\
        AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:\
        AES256-SHA256:AES128-SHA:AES256-SHA:DES-CBC3-SHA:!DSS"),
      options:         (ssl::SSL_OP_CIPHER_SERVER_PREFERENCE | ssl::SSL_OP_NO_COMPRESSION |
                         ssl::SSL_OP_NO_TICKET | ssl::SSL_OP_NO_SSLV2 |
                         ssl::SSL_OP_NO_SSLV3 | ssl::SSL_OP_NO_TLSV1).bits(),
      default_name:        Some(String::from("lolcatho.st")),
      default_app_id:      None,

      default_certificate: Some(Vec::from(&include_bytes!("../assets/certificate.pem")[..])),
      default_key:         Some(Vec::from(&include_bytes!("../assets/key.pem")[..])),
      default_certificate_chain: None,
    }
  }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Order {
    AddHttpFront(HttpFront),
    RemoveHttpFront(HttpFront),

    AddTlsFront(TlsFront),
    RemoveTlsFront(TlsFront),

    AddCertificate(CertificateAndKey),
    RemoveCertificate(CertFingerprint),

    AddTcpFront(TcpFront),
    RemoveTcpFront(TcpFront),

    AddInstance(Instance),
    RemoveInstance(Instance),

    HttpProxy(HttpProxyConfiguration),
    TlsProxy(TlsProxyConfiguration),

    SoftStop,
    HardStop,

    Status
}

impl Order {
  pub fn get_topics(&self) -> Vec<Topic> {
    match *self {
      Order::AddHttpFront(_)      => vec![Topic::HttpProxyConfig                       ],
      Order::RemoveHttpFront(_)   => vec![Topic::HttpProxyConfig                       ],
      Order::AddTlsFront(_)       => vec![Topic::TlsProxyConfig                        ],
      Order::RemoveTlsFront(_)    => vec![Topic::TlsProxyConfig                        ],
      Order::AddCertificate(_)    => vec![Topic::TlsProxyConfig                        ],
      Order::RemoveCertificate(_) => vec![Topic::TlsProxyConfig                        ],
      Order::AddTcpFront(_)       => vec![Topic::TcpProxyConfig                        ],
      Order::RemoveTcpFront(_)    => vec![Topic::TcpProxyConfig                        ],
      Order::AddInstance(_)       => vec![Topic::HttpProxyConfig, Topic::TlsProxyConfig, Topic::TcpProxyConfig],
      Order::RemoveInstance(_)    => vec![Topic::HttpProxyConfig, Topic::TlsProxyConfig, Topic::TcpProxyConfig],
      Order::HttpProxy(_)         => vec![Topic::HttpProxyConfig],
      Order::TlsProxy(_)          => vec![Topic::TlsProxyConfig],
      Order::SoftStop             => vec![Topic::HttpProxyConfig, Topic::TlsProxyConfig, Topic::TcpProxyConfig],
      Order::HardStop             => vec![Topic::HttpProxyConfig, Topic::TlsProxyConfig, Topic::TcpProxyConfig],
      Order::Status               => vec![Topic::HttpProxyConfig, Topic::TlsProxyConfig, Topic::TcpProxyConfig],
    }
  }
}

enum OrderField {
  Type,
  Data,
}


impl serde::Deserialize for OrderField {
  fn deserialize<D>(deserializer: &mut D) -> Result<OrderField, D::Error>
        where D: serde::de::Deserializer {
    struct OrderFieldVisitor;
    impl serde::de::Visitor for OrderFieldVisitor {
      type Value = OrderField;

      fn visit_str<E>(&mut self, value: &str) -> Result<OrderField, E>
        where E: serde::de::Error {
        match value {
          "type" => Ok(OrderField::Type),
          "data" => Ok(OrderField::Data),
          _ => Err(serde::de::Error::custom("expected type or data")),
        }
      }
    }

    deserializer.deserialize(OrderFieldVisitor)
  }
}

struct OrderVisitor;
impl serde::de::Visitor for OrderVisitor {
  type Value = Order;

  fn visit_map<V>(&mut self, mut visitor: V) -> Result<Order, V::Error>
        where V: serde::de::MapVisitor {
    let mut command_type:Option<String>    = None;
    let mut data:Option<serde_json::Value> = None;

    loop {
      match try!(visitor.visit_key()) {
        Some(OrderField::Type) => { command_type = Some(try!(visitor.visit_value())); }
        Some(OrderField::Data) => { data = Some(try!(visitor.visit_value())); }
        None => { break; }
      }
    }

    //println!("decoded type = {:?}, value= {:?}", command_type, data);
    let command_type = match command_type {
      Some(command) => command,
      None => try!(visitor.missing_field("type")),
    };

    // no data field for SoftStop and HardStop
    if &command_type == "SOFT_STOP" {
      try!(visitor.end());
      return Ok(Order::SoftStop);
    } else if &command_type == "HARD_STOP" {
      try!(visitor.end());
      return Ok(Order::HardStop);
    } else if &command_type == "STATUS" {
      try!(visitor.end());
      return Ok(Order::Status);
    }

    let data = match data {
      Some(data) => data,
      None       => try!(visitor.missing_field("data")),
    };

    try!(visitor.end());

    if &command_type == "ADD_HTTP_FRONT" {
      let res = serde_json::from_value(data).or(Err(serde::de::Error::custom("add_http_front")));
      //println!("ADD_HTTP_FRONT => {:?}", res);
      let acl = try!(res);
      Ok(Order::AddHttpFront(acl))
    } else if &command_type == "REMOVE_HTTP_FRONT" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("remove_http_front"))));
      Ok(Order::RemoveHttpFront(acl))
    } else if &command_type == "ADD_CERTIFICATE" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("add_certificate"))));
      Ok(Order::AddCertificate(acl))
    } else if &command_type == "REMOVE_CERTIFICATE" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("remove_certificate"))));
      Ok(Order::RemoveCertificate(acl))
    } else if &command_type == "ADD_TLS_FRONT" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("add_tls_front"))));
      Ok(Order::AddTlsFront(acl))
    } else if &command_type == "REMOVE_TLS_FRONT" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("remove_tls_front"))));
      Ok(Order::RemoveTlsFront(acl))
    } else if &command_type == "ADD_TCP_FRONT" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("add_tcp_front"))));
      Ok(Order::AddTcpFront(acl))
    } else if &command_type == "REMOVE_TCP_FRONT" {
      let acl = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("remove_tcp_front"))));
      Ok(Order::RemoveTcpFront(acl))
    } else if &command_type == "ADD_INSTANCE" {
      let instance = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("add_instance"))));
      Ok(Order::AddInstance(instance))
    } else if &command_type == "REMOVE_INSTANCE" {
      let instance = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("remove_instance"))));
      Ok(Order::RemoveInstance(instance))
    } else if &command_type == "CONFIGURE_HTTP_PROXY" {
      let conf = try!(serde_json::from_value(data).or(Err(serde::de::Error::custom("configure_http_proxy"))));
      Ok(Order::HttpProxy(conf))
    } else {
      Err(serde::de::Error::custom(format!("unrecognized command: {:?}", command_type)))
    }
  }
}

impl serde::Deserialize for Order {
  fn deserialize<D>(deserializer: &mut D) -> Result<Order, D::Error>
        where D: serde::de::Deserializer {
    static FIELDS: &'static [&'static str] = &["type", "data"];
    deserializer.deserialize_struct("Order", FIELDS, OrderVisitor)
  }
}

impl serde::Serialize for Order {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer,
  {
    let mut state = try!(serializer.serialize_map(Some(2)));

    match self {
      &Order::AddHttpFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "ADD_HTTP_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::RemoveHttpFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "REMOVE_HTTP_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::AddTlsFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "ADD_TLS_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::RemoveTlsFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "REMOVE_TLS_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::AddCertificate(ref certificate_and_key) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "ADD_CERTIFICATE"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, certificate_and_key));
      },
      &Order::RemoveCertificate(ref fingerprint) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "REMOVE_CERTIFICATE"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, fingerprint));
      },
      &Order::AddTcpFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "ADD_TCP_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::RemoveTcpFront(ref front) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "REMOVE_TCP_FRONT"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, front));
      },
      &Order::AddInstance(ref instance) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "ADD_INSTANCE"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, instance));
      },
      &Order::RemoveInstance(ref instance) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "REMOVE_INSTANCE"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, instance));
      },
      &Order::HttpProxy(ref config) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "CONFIGURE_HTTP_PROXY"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, config));
      },
      &Order::TlsProxy(ref config) => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "CONFIGURE_HTTP_PROXY"));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, config));
      },
      &Order::SoftStop => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "SOFT_STOP"));
      },
      &Order::HardStop => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "HARD_STOP"));
      },
      &Order::Status => {
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, "STATUS"));
      },
    }

    serializer.serialize_map_end(state)
  }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Topic {
    HttpProxyConfig,
    TlsProxyConfig,
    TcpProxyConfig
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;

  #[test]
  fn add_acl_test() {
    let raw_json = r#"{"type": "ADD_HTTP_FRONT", "data": {"app_id": "xxx", "hostname": "yyy", "path_begin": "xxx", "port": 4242}}"#;
    let command: Order = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}", command);
    assert!(command == Order::AddHttpFront(HttpFront{
      app_id: String::from("xxx"),
      hostname: String::from("yyy"),
      path_begin: String::from("xxx"),
    }));
  }

  #[test]
  fn remove_acl_test() {
    let raw_json = r#"{"type": "REMOVE_HTTP_FRONT", "data": {"app_id": "xxx", "hostname": "yyy", "path_begin": "xxx", "port": 4242}}"#;
    let command: Order = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}", command);
    assert!(command == Order::RemoveHttpFront(HttpFront{
      app_id: String::from("xxx"),
      hostname: String::from("yyy"),
      path_begin: String::from("xxx"),
    }));
  }


  #[test]
  fn add_instance_test() {
    let raw_json = r#"{"type": "ADD_INSTANCE", "data": {"app_id": "xxx", "ip_address": "yyy", "port": 8080}}"#;
    let command: Order = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}", command);
    assert!(command == Order::AddInstance(Instance{
      app_id: String::from("xxx"),
      ip_address: String::from("yyy"),
      port: 8080
    }));
  }

  #[test]
  fn remove_instance_test() {
    let raw_json = r#"{"type": "REMOVE_INSTANCE", "data": {"app_id": "xxx", "ip_address": "yyy", "port": 8080}}"#;
    let command: Order = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}", command);
    assert!(command == Order::RemoveInstance(Instance{
      app_id: String::from("xxx"),
      ip_address: String::from("yyy"),
      port: 8080
    }));
  }

  #[test]
  fn http_front_crash_test() {
    let raw_json = r#"{"type": "ADD_HTTP_FRONT", "data": {"app_id": "aa", "hostname": "cltdl.fr", "path_begin": ""}}"#;
    let command: Order = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}", command);
    assert!(command == Order::AddHttpFront(HttpFront{
      app_id: String::from("aa"),
      hostname: String::from("cltdl.fr"),
      path_begin: String::from(""),
    }));
  }

  #[test]
  fn http_front_crash_test2() {
    let raw_json = r#"{"app_id": "aa", "hostname": "cltdl.fr", "path_begin": ""}"#;
    let front: HttpFront = serde_json::from_str(raw_json).expect("could not parse json");
    println!("{:?}",front);
    assert!(front == HttpFront{
      app_id: String::from("aa"),
      hostname: String::from("cltdl.fr"),
      path_begin: String::from(""),
    });
  }
}

