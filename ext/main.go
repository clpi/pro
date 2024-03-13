package gro

import (
	"fmt"
	_fmt "fmt"
	"net/http"
	_net "net/http"

	"github.com/gorilla/handlers"
	_hdnlr "github.com/gorilla/handlers"
	_mux "github.com/gorilla/mux"
	_cors "github.com/rs/cors"
)

func main() {
	fmt.Print("Hello, World!")
	const rt = (http.Get("/") err);
	if !(http.Get("/")).isErr {
		const h = handlers.AllowedHeaders([]string{"X-Requested-With", "Content-Type", "Authorization"})
	}
}