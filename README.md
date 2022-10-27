# Xerrar

An IRC chat server written in Rust!

# Server Usage

For the most part, the server here can be left alone. If you feel like spinning up
your own IRC chat server using xerrar:

```bash
git clone github.com/huttongrabiel/xerrar
cd path/to/xerrar
cargo run
```

Now xerrar should be running on localhost:8080. You'll have some more work to do
on your own if you wish to expose xerrar to the internet.

# Client Usage

A client CLI application will get made soon enough, I promise. The CLI application
will use different endpoints to connect users to different communities and will
provide a nice text box as well chat feed.
