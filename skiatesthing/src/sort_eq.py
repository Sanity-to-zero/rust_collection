
with open("equations.txt", "r") as f:
    lines = f.readlines()
    collect = open("out.txt", "w")
    for l in lines:
        buff = ""
        #sort
        
        vars = l.split(";")[1]
        buff += l.split(";")[0]
        vars = sorted(vars)
        for s in vars:
            buff += s
        buff += l.split(";")[2]

        collect.write(buff)
    collect.close()


