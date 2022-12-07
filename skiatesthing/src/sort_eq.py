
with open("equations.txt", "r") as f:
    lines = f.readlines()
    collect = open("out.txt", "w")
    for l in lines:
        print(l)
        buff = ""
        #sort
        if l.strip() == "":
            collect.write("\n")
            continue
        else:
            vars = l.split(";")[1].split(" ")
            buff += l.split(";")[0] + ";"
            vars = sorted(vars)
            for s in vars:
                buff += " " + s +" "
            buff += ";"+ l.split(";")[2]
            collect.write(buff)
            print(buff)
    collect.close()


