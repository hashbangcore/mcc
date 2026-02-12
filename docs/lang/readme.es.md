# Netero

Un asistente de línea de comandos para modelos de lenguaje escrito en Rust, diseñado para flujos de trabajo centrados en la terminal.

**También disponible en inglés:** [See README in English](../../README.md)

## Estado del proyecto

Netero es un software experimental. Las características están incompletas y están sujetas a cambios.

## Variables de entorno

Netero puede configurarse dinámicamente mediante variables de entorno.

Proveedor por defecto:

* `CODE_API_KEY`
  Clave API utilizada para el proveedor `codestral` por defecto.

Proveedor personalizado (compatible con OpenAI):

* `NETERO_URL`
  URL completa del endpoint de chat completions.

* `NETERO_MODEL`
  Nombre del modelo enviado al proveedor.

* `NETERO_API_KEY`
  Clave API opcional para el proveedor personalizado.

`NETERO_URL` y `NETERO_MODEL` deben configurarse juntos. Si ambos están definidos, reemplazan al proveedor por defecto.

## Uso

Interfaz de línea de comandos para interactuar con modelos de lenguaje.

```
Uso: netero [OPCIONES] [PROMPT] [COMANDO]
```

Si se proporciona entrada a través de `stdin`, se utilizará como contexto adicional para el prompt.

### Comandos

* `chat`
  Abre una sesión de chat minimalista

* `commit`
  Genera un mensaje de commit a partir de los cambios en espera

* `prompt`
  Envía un prompt al modelo de lenguaje e imprime la respuesta

### Argumentos

* `[PROMPT]`
  Prompt pasado al modelo de lenguaje

### Opciones

* `-v, --verbose`
  Habilita la salida detallada

* `-h, --help`
  Muestra la ayuda

* `-V, --version`
  Muestra la versión

## Ejemplos

### Prompt básico

```sh
netero "Explica la diferencia entre enlaces duros y simbólicos"
```

### Usando stdin para prompts más largos

```sh
cat README.md | netero "Resume el README del proyecto"
```

### Generar un mensaje de commit de Git

```sh
netero commit | git commit -F - --edit
```

### Usando un proveedor personalizado

```sh
export NETERO_URL="https://api.example.com/v1/chat/completions"
export NETERO_MODEL="mi-modelo"
export NETERO_API_KEY="tu-api-key"
netero "Explica cómo systemd gestiona los servicios"
```

### Salida detallada

```sh
netero -v "Explica el modelo de propiedad de Rust"
```

### Procesar una página de manual

```sh
man tmux | netero "¿Cómo puedo dividir una ventana de tmux?"
```

### Analizar la salida de un comando

```sh
ps aux | netero "¿Qué procesos están consumiendo más recursos?"
```

### Enviar la salida a otro comando
```sh
ss -tulpen | netero "Resume los sockets de escucha activos" | mdless
```

## Licencia

BSD 2-Clause
