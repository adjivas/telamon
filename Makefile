MAKEFLAGS += -s

TARGET = poc

# http://westes.github.io/flex/manual/Makefiles-and-Flex.html#Makefiles-and-Flex
LEX = flex

# 16.3 Variables for Specifying Commands § https://www.gnu.org/software/make/manual/make.html#Command-Variables
CFLAGS += -Werror -Wextra -Werror
ALL_CFLAGS = -D ECHO -I. $(CFLAGS)

LDFLAGS += -lfl

INDENTOPT = -kr -di 4 -bli0 -bls -sob -bad

all:
	$(LEX) -o$(TARGET).c $(TARGET).l
	$(CC) $(ALL_CFLAGS) -o $(TARGET) $(TARGET).c $(LDFLAGS)

clean:
	rm -v -fr $(TARGET).c

fclean: clean
	rm -v -fr $(TARGET)

re: fclean all
