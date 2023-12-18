PLUGIN_NAME = docker
USER_PATH = $(HOME)/.local/share/pop-launcher/plugins/${PLUGIN_NAME}
SYSTEM_PATH = /etc/pop-launcher/plugins/${PLUGIN_NAME}
PHONY := test_user test_system install_user install_system uninstall_user uninstall_system

test:
    cargo check
    cargo clippy
    cargo test

build_test: test
    cargo build
build_release: test
    cargo clean
    cargo build --release

cp_ron_user:
	install -Dm0764 $(realpath .)/plugin.ron ${USER_PATH}/plugin.ron
	install -Dm0764 $(realpath .)/plugin-ps.ron ${USER_PATH}-ps/plugin.ron
cp_ron_system:
	install -Dm0774 $(realpath .)/plugin.ron ${SYSTEM_PATH}/plugin.ron
	install -Dm0774 $(realpath .)/plugin.ron ${SYSTEM_PATH}-ps/plugin.ron

test_user: build_test cp_ron_user
    install -Dm0754 $(realpath target/debug)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
test_system: build_test cp_ron_system
    install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}

install_user: build_release cp_ron_user
    install -Dm0754 $(realpath target/release)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
install_system: build_release cp_ron_system
    install -Dm0774 $(realpath target/release)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}

uninstall_user:
    rm -r ${USER_PATH}
uninstall_system:
    rm -r ${SYSTEM_PATH}
