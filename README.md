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