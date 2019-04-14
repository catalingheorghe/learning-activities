#/bin/sh

for i in {1..100};
	do echo "(${RANDOM:0:3}) ${RANDOM:0:3}-${RANDOM:0:4}" >> phonelist.txt;
done


