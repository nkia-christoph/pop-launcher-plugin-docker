PLUGIN_NAME = docker
USER_PATH = $(HOME)/.local/share/pop-launcher/plugins/${PLUGIN_NAME}
SYSTEM_PATH = /etc/pop-launcher/plugins/${PLUGIN_NAME}
PHONY := test_user test_system install_user install_system uninstall_user uninstall_system

test:
    cargo check
    cargo test

build_test: test
    cargo build
build_release: test
    cargo clean
    cargo build --release

test_user: build_test
    install -Dm0754 $(realpath target/debug)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
test_system: build_test
    install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}

install_user: build_release
    install -Dm0754 $(realpath target/release)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
install_system: build_release
    install -Dm0774 $(realpath target/release)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}

uninstall_user:
    rm -r ${USER_PATH}
uninstall_system:
    rm -r ${SYSTEM_PATH}
