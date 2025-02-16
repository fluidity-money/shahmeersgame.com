// Code generated by github.com/99designs/gqlgen, DO NOT EDIT.

package model

type Idea struct {
	// Timestamp of this creation.
	Time int32 `json:"time"`
	// Description of this idea.
	Desc string `json:"desc"`
	// Hash of this idea in its submitted form.
	Hash string `json:"hash"`
	// Submitter of this idea's address. We track this for internal use sake, we don't actually
	// track if this was created, or if someone lied about the creation. We won't even display
	// this to the UI.
	Submitter string `json:"submitter"`
}

type Mutation struct {
}

type Query struct {
}
