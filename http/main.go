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
	switch r.Method {
	case http.MethodPost:
		fmt.Fprintln(w, "Hello")
	default:
		h.unsupportedMethod(w, r)
	}
}

func (h *Handler) unsupportedMethod(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusMethodNotAllowed)
}
