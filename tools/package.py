from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
root=Path(__file__).resolve().parents[1]
out=root/'dist/MHGU_PC_Monster_Teleport_v0.1.0.zip'
out.parent.mkdir(exist_ok=True)
with ZipFile(out,'w',ZIP_DEFLATED) as z:
    z.write(root/'monster_teleport.nro','romfs/nativeNX/plugins/monster_teleport.nro')
    z.write(root/'说明.txt','说明.txt')
print(out)
