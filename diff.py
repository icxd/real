import difflib
 
with open('generated.txt') as file_1:
    file_1_text = file_1.readlines()
 
with open('generated2.txt') as file_2:
    file_2_text = file_2.readlines()
 
for line in difflib.unified_diff(
        file_1_text, file_2_text, fromfile='generated.txt',
        tofile='generated2.txt', lineterm=''):
    print(line)