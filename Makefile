MAKEFLAGS += -s

TARGET = poc

# http://westes.github.io/flex/manual/Makefiles-and-Flex.html#Makefiles-and-Flex
LEX = flex

# 16.3 Variables for Specifying Commands § https://www.gnu.org/software/make/manual/make.html#Command-Variables
CFLAGS += -Werror -Wextra -Werror
ALL_CFLAGS = -D ECHO -I. $(CFLAGS)

LDFLAGS += -lfl

all: static dynamic 

static: lex
	$(CC) $(ALL_CFLAGS) -o $(TARGET).o $(TARGET).c $(LDFLAGS)
	ar rcs $(TARGET).a $(TARGET).o

dynamic: lex
	$(CC) -fPIC -shared $(ALL_CFLAGS) -o $(TARGET).so $(TARGET).c $(LDFLAGS)

lex:
	$(LEX) -o$(TARGET).c $(TARGET).l

clean:
	rm -v -fr $(TARGET).o
	rm -v -fr $(TARGET).c

fclean: clean
	rm -v -fr $(TARGET).a
	rm -v -fr $(TARGET).so

re: fclean all
