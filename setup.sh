#!/bin/bash


install_plantuml() {
echo "Install /usr/local/bin/plantuml"

echo '
#!/bin/sh

java -jar /opt/plantuml/plantuml.jar "$@"
' > /usr/local/bin/plantuml 
chmod 555 /usr/local/bin/plantuml 

mkdir -p /opt/plantuml
cp plantuml*.jar /opt/plantuml
ln -s /opt/plantuml/*.jar /opt/plantuml/plantuml.jar

echo '
@startuml
Alice -> Bob: test
@enduml
' > /tmp/uml.txt

# java -jar plantuml.jar /tmp/uml.txt
/usr/local/bin/plantuml /tmp/uml.txt

rm /tmp/uml.txt

}

install_plantuml


