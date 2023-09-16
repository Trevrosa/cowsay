# cowsay
host an api for the linux command `cowsay` on port 8000

# endpoints
/cowsay/\<input\>/ -> `cowsay "input"`

/cowsay/\<input\>/\<kind\>/ -> `cowsay -f "kind" "input"`
