package main

import (
	"fmt"
	"html/template"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		RenderTemplate(w, "index")
	})
	fmt.Print("Starting server on localhost:5000")
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))
	http.ListenAndServe(":5000", nil)
}

func RenderTemplate(w http.ResponseWriter, page_name string) {
	template, error := template.ParseFiles("static/templates/" + page_name + ".html")
	if error != nil {
		http.Error(w, error.Error(), http.StatusInternalServerError)
		return
	}
	template.Execute(w, nil)
}
