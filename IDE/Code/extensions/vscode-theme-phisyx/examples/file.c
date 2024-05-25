+ - * \ / % = < > | & ! % "" '' # () [] {} , ;

int
unsigned int
void

const 
static

NULL

enum x {
    A = 0,
    B,
};

struct x;
sizeof(x);

union x {
    union {
		char *y;
    }
}

typedef struct {
    union	 x xx;
	char	*y;
	enum	 z zz;
} xyz;

#define _ 
#include ""

// -----

#include <unistd.h>
#include "file.h"

#ifndef XX
#endif // XX

#if 0
    // Commentaire
#else 
    /**
     * Commentaire
     */
#endif

#define CRLF "\r\n"


typedef void (*fn)();

typedef struct list
{
	char name[MAX_SIZE + 1];
	void* data;
	struct list* next;
} ty_list;

void
fn(ty_list* current)
{
    while (current)
    {
        if (((ty_list*)current->data) == current)
        {
            print("Hello World\0");
            continue;
        }
        
        break;
    }

    free((ty_list*)current->data);
    free(current);
}

int 
main(int argc, char *argv[])
{
	ty_server server;

	create_server(&server);

	while (1)
	{
		initialize_all_fd(&server);

		if (select(server.fd_max + 1,
				   &(server.fd_read),
				   &(server.fd_write),
				   0,
				   &(server.timeout)) < 0)
		{
			printf("Erreur: select\n");
		}

		read_all_fd(&server);
	}

	return EXIT_SUCCESS;
}
