DIR="$( dirname "${BASH_SOURCE[0]}" )"

set -e

docker build --rm -t flategy-rust -f "${DIR}/app.dockerfile" "${DIR}"

docker run --rm -it --user "$(id -u)":"$(id -g)" -v "$PWD":/work -w /work -p 8000:8000 flategy-rust "$@"
