PORT="${1:-8888}"
DIR="$( dirname "${BASH_SOURCE[0]}" )"

set -e

docker build --rm -t flategy-notebook -f "${DIR}/notebook.dockerfile" "${DIR}"

docker run --rm -it --user "$(id -u)":"$(id -g)" --group-add users \
       -v "$PWD":/work -w /work -p "${PORT}:${PORT}" \
       flategy-notebook \
       jupyter lab --allow-root --ip "*" --port "${PORT}"
