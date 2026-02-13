# Netero

Un asistente LLM para la terminal, pensado para usuarios avanzados.

**También en inglés:** [Ver README en inglés](../../README.md)

## Estado del proyecto

Experimental. Las funciones están incompletas y pueden cambiar.

## Variables de entorno

Configura con variables de entorno.

Proveedor por defecto (`codestral`):

* `CODE_API_KEY`
  Clave API del proveedor por defecto.

Proveedor personalizado (compatible con OpenAI):

* `NETERO_URL`
  URL del endpoint de chat completions.

* `NETERO_MODEL`
  Nombre del modelo enviado al proveedor.

* `NETERO_API_KEY`
  Clave API opcional para el proveedor personalizado.

`NETERO_URL` y `NETERO_MODEL` deben ir juntos. Si están definidos, reemplazan al proveedor por defecto.

## Uso

CLI para interactuar con modelos de lenguaje.

```
Uso: netero [OPCIONES] [PROMPT] [COMANDO]
```

Si hay entrada por `stdin`, se agrega como contexto extra.
Las expresiones `$(...)` se ejecutan en tu shell y su salida se añade al prompt.

### Comandos

* `chat`
  Abre una sesión de chat minimalista

* `commit`
  Genera un mensaje de commit a partir de los cambios en espera

* `completion`
  Genera scripts de autocompletado para el shell

* `prompt`
  Envía un prompt e imprime la respuesta

#### Comandos del chat

* `/help`
  Muestra la ayuda

* `/clean`
  Limpia el historial del chat

* `/trans`
  Traduce texto

* `/eval`
  Evalúa una expresión aritmética

### Argumentos

* `[PROMPT]`
  Prompt enviado al modelo

### Opciones

* `-v, --verbose`
  Habilita la salida detallada

* `-h, --help`
  Muestra la ayuda

* `-V, --version`
  Muestra la versión

## Ejemplos

### Prompts con y sin comillas

```sh
netero Explica la diferencia entre enlaces duros y simbólicos
netero "Explica la diferencia entre enlaces duros y simbólicos"
netero -v ¿Qué es Docker?
netero -v "¿Qué es Docker?"
```

### Usando stdin para prompts más largos

```sh
cat README.md | netero Resume este README
cat README.md | netero "Resume este README"
```

### Generar un mensaje de commit de Git

```sh
netero commit | git commit -F - --edit
```

### Generar autocompletado para el shell

```sh
netero completion bash
```

### Usando un proveedor personalizado

```sh
export NETERO_URL="https://api.example.com/v1/chat/completions"
export NETERO_MODEL="mi-modelo"
export NETERO_API_KEY="tu-api-key"
netero Explica cómo systemd gestiona los servicios
```

### Salida detallada

```sh
netero -v ¿Qué es Docker?
netero -v "¿Qué es Docker?"
```

### Ejecutar comandos en línea dentro de chat

```sh
netero chat
# luego escribe:
Resume este directorio: $(ls -la)
```

### Comandos del chat

```sh
netero chat
# luego escribe:
/help
/eval 2 + 2 * 5
/trans This is a test
/clean
```

### Procesar una página de manual

```sh
man tmux | netero ¿Cómo puedo dividir una ventana en paneles?
```

### Analizar la salida de un comando

```sh
ps aux | netero ¿Qué consume más recursos?
```

### Enviar la salida a otro comando
```sh
ss -tulpen | netero Dime los sockets activos | glow -p
```

## Licencia

BSD 2-Clause
