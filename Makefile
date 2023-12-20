PLUGIN_NAME = docker
USER_PATH = $(HOME)/.local/share/pop-launcher/plugins/${PLUGIN_NAME}
SYSTEM_PATH = /etc/pop-launcher/plugins/${PLUGIN_NAME}
DIST_PATH = /usr/lib/pop-launcher/plugins/${PLUGIN_NAME}
PHONY := test_user test_system test_dist install_user install_system install_dist uninstall_user uninstall_system uninstall_dist

test:
    cargo check
    cargo clippy
    cargo test

build_test: test
    cargo build
build_release: test
    cargo clean
    cargo build --release

cp_data_user:
	install -Dm0764 $(realpath .)/plugins/${PLUGIN_NAME} ${USER_PATH}
	install -Dm0764 $(realpath .)/plugins/${PLUGIN_NAME}-ps ${USER_PATH}-ps
cp_data_system:
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME} ${SYSTEM_PATH}
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}-ps ${SYSTEM_PATH}-ps
cp_data_dist:
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME} ${DIST_PATH}
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}-ps ${DIST_PATH}-ps

test_user: build_test cp_data_user
    install -Dm0754 $(realpath target/debug)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
test_system: build_test cp_data_system
    install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}
test_dist: build_test cp_data_dist
    install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${DIST_PATH}/${PLUGIN_NAME}

install_user: build_release cp_data_user
    install -Dm0754 $(realpath target/release)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}
install_system: build_release cp_data_system
    install -Dm0774 $(realpath target/release)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}
install_dist: build_release cp_data_dist
    install -Dm0774 $(realpath target/release)/${PLUGIN_NAME} ${DIST_PATH}/${PLUGIN_NAME}

uninstall_user:
    rm -r ${USER_PATH}
uninstall_system:
    rm -r ${SYSTEM_PATH}
uninstall_dist:
    rm -r ${DIST_PATH}
