#!/bin/bash
echo "Visit:"
echo "Install Rust"
echo https://www.rust-lang.org/tools/install
echo "Install github official gh command line tool"
echo https://github.com/cli/cli/blob/trunk/docs/install_linux.md
echo "Install vscode"
echo https://linuxize.com/post/how-to-install-visual-studio-code-on-ubuntu-20-04/

echo 
echo "Apt packages to install (compiler, java, build tools, rust dependencies)"
echo sudo apt install build-essential
echo sudo apt install default-jre
echo sudo apt install cmake
echo cargo install cargo-edit
echo cargo install mdbook
echo cargo install mdbook-plantuml

echo "Other optional stuff to install - SEE INSTALL.md"

echo -n "PRESS ENTER AFTER YOU HAVE INSTALLED THESE"; read x

install_plantuml() {
    echo "Download plantuml.jar"
    echo -n "PRESS ENTER"; read x
    echo https://github.com/plantuml/plantuml/releases/latest
    wget -v -O /tmp/plantuml.jar https://github.com/plantuml/plantuml/releases/download/v1.2022.0/plantuml-1.2022.0.jar

    echo "_______________________________________________________"
    echo "Create /usr/local/bin/plantuml wrapper script"
    echo -n "PRESS ENTER"; read x
    echo '#!/bin/sh' > /tmp/plantuml
    echo 'java -jar /opt/plantuml/plantuml.jar "$@"' >> /tmp/plantuml

    echo '#!/bin/sh' > /tmp/plantuml_install
    echo "cp /tmp/plantuml /usr/local/bin/plantuml" >> /tmp/plantuml_install
    echo "chmod 555 /usr/local/bin/plantuml" >> /tmp/plantuml_install
    echo "mkdir -p /opt/plantuml" >> /tmp/plantuml_install
    echo "cp /tmp/plantuml.jar /opt/plantuml" >> /tmp/plantuml_install
    echo "ln -s /opt/plantuml/*.jar /opt/plantuml/plantuml.jar" >> /tmp/plantuml_install

    echo "_______________________________________________________"
    ls -l /tmp/plantuml*
    echo "_______________________________________________________"
    echo "/tmp/plantuml"
    cat /tmp/plantuml
    echo "_______________________________________________________"
    echo "/tmp/plantuml_install"
    cat /tmp/plantuml_install
    echo "_______________________________________________________"

    echo "Run: sudo sh -x /tmp/plantuml_install"
}


test_install_plantuml() {
    echo "test install"
    echo '
@startuml
Alice -> Bob: test
@enduml
' > /tmp/uml.txt

    echo "_______________________________________________________"
    cat /tmp/uml.txt
    echo "_______________________________________________________"
    echo "[running] /usr/local/bin/plantuml /tmp/uml.txt"
    /usr/local/bin/plantuml /tmp/uml.txt
    rm /tmp/uml.txt
    echo "[viewing] eog /tmp/uml.png"
    eog /tmp/uml.png &
}

#install_plantuml
test_install_plantuml


