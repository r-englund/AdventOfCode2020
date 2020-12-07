import sys
import os

if len(sys.argv) != 2: 
    print("Usage: {} [day as a int]".format(sys.argv[0]))
    exit()


day = int(sys.argv[1])
if day < 1 or day > 25:
    print("Incorrect day, should be between 1 and 25")
    exit()


daystr = "day{:02}".format(day)
rsfile = "src/bin/{}.rs".format(daystr)
inputfile = "src/bin/{}-input.txt".format(daystr)

src = """/*
PLACEHOLDER_FOR_INSTRUCTIONS
*/

/*
PLACEHOLDER_FOR_INSTRUCTIONS_PART_2
*/

static INPUT: &str = include_str!("{}-input.txt");

fn main() {{

}}
""".format(daystr)

if os.path.exists(rsfile):
    print("File {} already exists, exiting without doing anythning".format(rsfile))
    exit()

with open(inputfile,"w") as f:
    pass
    
with open(rsfile,"w") as f:
    f.write(src)
    

url_instr = 'https://adventofcode.com/2020/day/{}'.format(day)
url_input = 'https://adventofcode.com/2020/day/{}/input'.format(day)
print("Find todays instructions at: {}".format(url_instr))
print("Find todays input at: {}".format(url_input))


