# Docker
_Pop Launcher Plugin written in Rust_

**Manage your docker or compose containers** from the **Pop!_OS Launcher**!

## Roadmap 🚀
**until v0.1.0 release 🗻***
- [ ] 🗻 base functionality on cmd line docker command
- [ ] 🐳 docker
    - [ ] list containers (ps)
    - [ ] stop (1/all)
    - [ ] restart (1/all)
- [ ] 🌐 compose
    - [ ] list containers (ps)
    - [ ] start container (1/all)
    - [ ] stop (1/all)
    - [ ] restart (1/all)

**until v1.0.0 release 🎯**
- [ ] 📄 show logs
    - [ ] last 10 lines (default)
    - [ ] since n minutes (in terminal)
    - [ ] add `-f` flag (launcher or terminal)
- [ ] ✂️ copy to clipboard
    - [ ] container name
    - [ ] container id
- [ ] 📨 1-off command through launcher (exec)
- [ ] 🔗 attach shell in terminal (exec -i)
- [ ] 🗑️ remove container
- [ ] 🔄 rebuild container
    - [ ] without deps
- [ ] 👀 hints galore

**until v2.0.0 release 💎**
- [ ] 🏇 base functionality on docker socket communication
- [ ] 👥 enable connection to remote docker through ssh

---

**nice to have 💁‍♂️**
- [ ] 😮 interactive cp
- [ ] images
- [ ] prune
- [ ] volumes
- [ ] network
- [ ] secret
- [ ] search
- [ ] version
- [ ] stats

---

## Debugging
see standard launcher log location:
```
~/.local/state/pop-launcher/pop-launcher.log
~/.local/state/pop-launcher/docker.log
```

---

This plugin was created due to the mild frustration experienced with the official vscode docker extension.



// ♻ U+267B
// ↻ U+21BB
// dc up: ⇈ U+21C8
// dc down: ⇊ U+21CA
// prune: ♻ U+267B
// attach: ⎆ U+2386
// logs: ⎙ U+2399



    pub async fn view_down_notice(&mut self) {
        info!(" - view down notice");

        let result = PluginSearchResult {
            id: 0 as Indice,
            name: "Docker is not running".to_owned(),
            description: "Would you like to start it?".to_owned(),
            icon: icon_borrowed!("dialog-error"),
            //category_icon: self.icon.to_owned(),
            ..Default::default()
        };
        tokio::spawn( async move {
            send(&mut async_stdout(),PluginResponse::Append(result)).await;
        });
    }
