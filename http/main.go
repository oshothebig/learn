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
		h.Post(w, r)
	default:
		h.unsupportedMethod(w, r)
	}
}

func (h *Handler) Post(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello")
}

func (h *Handler) unsupportedMethod(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusMethodNotAllowed)
}
