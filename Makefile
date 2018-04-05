MAKEFLAGS += -s

TARGET := poc

# 16.3 Variables for Specifying Commands § https://www.gnu.org/software/make/manual/make.html#Command-Variables
CFLAGS = -Werror -Wextra -Werror
ALL_CFLAGS = -I. $(CFLAGS)

LDFLAGS += -lfl

INDENTOPT = -kr -di 4 -bli0 -bls -sob -bad

all:
	lex -o$(TARGET).c $(TARGET).l
	cc $(ALL_CFLAGS) -o $(TARGET) $(TARGET).c $(LDFLAGS)

indent: $(SOURCE)
	indent $(INDENTOPT) $^

clean:
	rm -v -fr $(TARGET).c

fclean: clean
	rm -v -fr $(TARGET)

re: fclean all
