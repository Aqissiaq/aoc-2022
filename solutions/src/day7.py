def calculate_size(dir):
    size = 0
    for _, content in dir.items():
        if type(content) == int:
            size += content
        else:
            size += calculate_size(content)
    return size

def get_all_dirs(root):
    if not isinstance(root, dict):
        return []

    all = [root]
    for _, content in root.items():
        if isinstance(content, dict):
            inner = get_all_dirs(content)
            all += inner
    return all

def print_fs(root, indent):
    spaces = ' '*indent
    for k, i in root.items():
        if isinstance(i, int):
            print(f"{spaces}{k} - {i}")
        else:
            print(f"{spaces}{k}")
            print_fs(i, indent + 2)

TOTAL_SIZE = 70_000_000
REQUIRED_SIZE = 30_000_000

if __name__ == "__main__":
    # with open("test.txt", "r") as f:
    #     data = list(f.readlines())
    with open("../../inputs/day7", "r") as f:
        data = list(f.readlines())

    root = {}
    fs = {"/": root}
    cwd = [fs]

    for line in data:
        line = line.strip()
        if line[:4] == "$ cd":
            name = line[5:]
            if name == "..":
                cwd.pop()
            else:
                cwd.append(cwd[-1][name])
        elif line[:4] == "$ ls":
            continue
        elif line[:3] == "dir":
            name = line[4:]
            cwd[-1][name] = {}
        else:
            size, name = line.split()
            cwd[-1][name] = int(size)

    current_free = TOTAL_SIZE - calculate_size(root)
    required_del = REQUIRED_SIZE - current_free

    candidate = TOTAL_SIZE
    for d in get_all_dirs(fs):
        size = calculate_size(d)
        if required_del <= size <= candidate:
            candidate = size

    print(candidate)
