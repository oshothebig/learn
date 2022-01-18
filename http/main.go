package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.Handle("/", &Handler{})
	http.ListenAndServe(":8080", nil)
}

type Handler struct{}

func (h *Handler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello")
}
