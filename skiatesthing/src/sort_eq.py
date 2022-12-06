
with open(".txt", "r") as f:
    lines = f.readlines()
    collect = open(".txt", "w")
    for l in lines:
        buff = ""
        #sort
        if l == "":
            vars = l.split(";")[1]
            vars = sorted(vars)
            for s in vars:
                buff += s
        collect.write(buff)


