use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, fs};

const INDEX: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
      <meta charset=\"UTF-8\">
      <title>Download</title>
  </head>

  <style>
    a {
      background-color: #2980b9;
      color: white;
      text-decoration: none;
      padding: 5px 10px;
      border-radius: 5px;
      transition: background-color 0.3s ease;
    }

    a:hover {
      background-color: #21a4fa;
    }
  </style>

  <body>
    <h1>Files in /files</h1>
    {}
  </body>
</html>";

const PAGE_404: &str = "<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <title>404</title>
    </head>
    <body>
        <h1>404 Not Found</h1>
    </body>
</html>";


fn main()
{
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2
    {
        eprintln!("No ip address given.");
        return;
    }
    else if args.len() > 3
    {
        eprintln!("Too many arguments.");
        return;
    }
    if ["-h", "--help", "help"].contains(&args[1].as_str())
    {
        println!("File sharer through tcp. Connect to ip on any device and download files from /files or inputted path.

Usage: file-sharer-tcp <COMMANDS>

Commands:
    <ip>                Serves /files at <ip>.
    <ip> <path>         Serves folder at <path> at <ip>.
    help | -h | --help  Prints this message.
");
        return;
    }

    let address = &args[1];
    let dir = if args.len() == 3 {
        args[2].clone()
    } else
    {
        String::from("files")
    };

    if let Ok(listener) = TcpListener::bind(format!("{address}:7878"))
    {
        println!("Serving at: http://{}", listener.local_addr().unwrap());
        println!("Ctrl-C to quit.");

        if let Ok(mut server) = Server::new(dir.clone())
        {
            server.listen(listener);
        }
        else
        {
            eprintln!("Invalid folder path: {}.", dir);
        }
    }
    else
    {
        eprintln!("Invalid ip address: {}.", address);
    }
}


pub struct Server
{
    paths: Vec<String>,
    dir: String,
}

impl Server
{
    pub fn new(dir: String) -> std::io::Result<Server>
    {
        let _ = fs::read_dir(&dir)?;

        Ok(Server {
            paths: Vec::new(),
            dir,
        })
    }

    pub fn listen(&mut self, listener: TcpListener)
    {
        for stream in listener.incoming()
        {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    pub fn handle_connection(&mut self, mut stream: TcpStream)
    {
        let buf_reader = BufReader::new(&mut stream);

        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1"
        {
            let status_line = "HTTP/1.1 200 OK";
            let mut contents = INDEX.to_string();

            let mut downloads = String::new();

            let dir = fs::read_dir(&self.dir).unwrap();
            for entry in dir
            {
                let entry = entry.unwrap();
                self.paths.push(entry.path().display().to_string());
                downloads.push_str(format!("<a href=\"/{path}\">{path}</a><br><br>\r\n", path=entry.path().to_str().unwrap()).as_str());
            }

            contents = contents.replace("{}", &downloads);

            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        else if self.paths.contains(&request_line[5..request_line.find(" HTTP/1.1").unwrap()].to_string().replace("/","\\"))
        {
            let path = request_line[5..request_line.find(" HTTP/1.1").unwrap()].to_string().replace("/","\\");

            let start = path.rfind("\\").unwrap();
            let name = &path[start..];

            let mut file = fs::File::options().create(false).write(false).read(true).open(&path).unwrap();
            let mut content = Vec::new();

            let length = file.read_to_end(&mut content).unwrap();

            let mut headers = Headers::new();

            headers.add("Pragma", "public");
            headers.add("Cache-Control", "private");
            headers.add("Content-Disposition", format!("attachment; filename=\"{}\"", name).as_str());
            headers.add("Content-Transfer-Encoding", "binary");
            headers.add("Content-Length", length.to_string().as_str());

            let response = format!("HTTP/1.1 200 OK\r\n{}\r\n", headers.make());

            let mut response = response.as_bytes().to_vec();
            response.extend(content);

            stream.write_all(response.as_slice()).unwrap();
        }
        else
        {
            Self::handle_404(stream);
        }
    }

    pub fn handle_404(mut stream: TcpStream)
    {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = PAGE_404.to_string();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}


struct Headers
{
    headers: Vec<String>,
}

impl Headers
{
    pub fn new() -> Headers
    {
        Headers {
            headers: Vec::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str)
    {
        self.headers.push(format!("{}: {}\r\n", key, value));
    }

    pub fn make(&self) -> String
    {
        self.headers.join("")
    }
}