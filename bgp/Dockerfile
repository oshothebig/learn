FROM frrouting/frr:v8.4.0

RUN sed -i -e 's/bgpd=no/bgpd=yes/g' /etc/frr/daemons
COPY vtysh.conf /etc/frr/