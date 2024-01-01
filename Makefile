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

ensure_dir_user:
	mkdir -p ~${USER_PATH}
	mkdir -p ~${USER_PATH}-ps

ensure_dir_system:
	mkdir -p ~${SYSTEM_PATH}
	mkdir -p ~${SYSTEM_PATH}-ps

ensure_dir_dist:
	mkdir -p ~${DIST_PATH}
	mkdir -p ~${DIST_PATH}-ps

cp_data_user: ensure_dir_user
	install -Dm0744 $(realpath .)/plugins/${PLUGIN_NAME}/docker-icon.png ${USER_PATH}/docker-icon.png
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}/plugin.ron ${USER_PATH}/plugin.ron
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}-ps/plugin.ron ${USER_PATH}-ps/plugin.ron

cp_data_system: ensure_dir_system
	install -Dm0744 $(realpath .)/plugins/${PLUGIN_NAME}/docker-icon.png ${SYSTEM_PATH}/docker-icon.png
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}/plugin.ron ${SYSTEM_PATH}/plugin.ron
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}-ps/plugin.ron ${SYSTEM_PATH}-ps/plugin.ron

cp_data_dist: ensure_dir_dist
	install -Dm0744 $(realpath .)/plugins/${PLUGIN_NAME}/docker-icon.png ${DIST_PATH}/docker-icon.png
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}/plugin.ron ${DIST_PATH}/plugin.ron
	install -Dm0774 $(realpath .)/plugins/${PLUGIN_NAME}-ps/plugin.ron ${DIST_PATH}-ps/plugin.ron

test_user: build_test cp_data_user
	install -Dm0755 $(realpath target/debug)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}

test_system: build_test cp_data_system
	install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${SYSTEM_PATH}/${PLUGIN_NAME}

test_dist: build_test cp_data_dist
	install -Dm0774 $(realpath target/debug)/${PLUGIN_NAME} ${DIST_PATH}/${PLUGIN_NAME}

install_user: build_release cp_data_user
	install -Dm0755 $(realpath target/release)/${PLUGIN_NAME} ${USER_PATH}/${PLUGIN_NAME}

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
