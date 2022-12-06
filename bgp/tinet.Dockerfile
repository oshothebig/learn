FROM golang:1.19.3 as build

WORKDIR /build
RUN go install github.com/tinynetwork/tinet@latest

FROM scratch as export
COPY --from=build /go/bin/tinet /
