```                                                                                     
/$$$$$$$  /$$$$$$$   /$$$$$$  /$$   /$$           /$$ /$$                                
| $$__  $$| $$__  $$ /$$__  $$| $$  | $$          | $$| $$                               
| $$  \ $$| $$  \ $$| $$  \__/| $$  | $$  /$$$$$$ | $$| $$  /$$$$$$   /$$$$$$  /$$$$$$   
| $$$$$$$/| $$$$$$$/| $$ /$$$$| $$$$$$$$ /$$__  $$| $$| $$ /$$__  $$ /$$__  $$/$$__  $$  
| $$__  $$| $$____/ | $$|_  $$| $$__  $$| $$$$$$$$| $$| $$| $$  \ $$| $$$$$$$$| $$  \__/ 
| $$  \ $$| $$      | $$  \ $$| $$  | $$| $$_____/| $$| $$| $$  | $$| $$_____/| $$       
| $$  | $$| $$      |  $$$$$$/| $$  | $$|  $$$$$$$| $$| $$| $$$$$$$/|  $$$$$$$| $$       
|__/  |__/|__/       \______/ |__/  |__/ \_______/|__/|__/| $$____/  \_______/|__/       
                                                         | $$                            
    A Discord bot to help RPGs by Steely (SteelStone)    | $$    1.1.0-BETA              
                                                         |__/                            
                                                                                         
* Art made in: https://www.asciiart.eu/text-to-ascii-art (Using "Big Money-ne")          
```
# Funções:
> [!TIP]
> O bot desconsidera a diferença entre minúsculas e maiúsculas, ambas terão o mesmo resultado. <br>
> O bot também desconsidera espaços.

## Funções de dados:
> [!IMPORTANT]
> Nenhuma função de dados aceita números negativos ou fracionários.

### `D` (Die)
Escolhe um número aleatório entre 1 e o número posterior x vezes e soma todos os resultados. <br>
A quantidade de repetições é definida pelo número anterior, caso não haja nenhum, ele só repetirá uma vez.

### `A` (Apply die)
Escolhe um número aleatório entre 1 e o número posterior e deixa a conta continuar. <br>
Após dado o resultado ele fará isso novamente x vezes, dando diversos resultados diferentes. <br>
A quantidade de repetições é definida pelo número anterior, caso não haja nenhum, ele só repetirá uma vez.
> [!CAUTION]
> Não implementado. Pode ser alterado ou até removido.

### `L` (Lower die)
Escolhe um número aleatório entre 1 e o número posterior x vezes e escolhe o menor resultado. <br>
A quantidade de repetições é definida pelo número anterior, caso não haja nenhum, ele só repetirá uma vez.

### `G` (Greater die)
Escolhe um número aleatório entre 1 e o número posterior x vezes e escolhe o maior resultado. <br>
A quantidade de repetições é definida pelo número anterior, caso não haja nenhum, ele só repetirá uma vez.

## Funções Matemáticas:

### `+` (Soma)
Soma o número anterior pelo proximo.

### `-` (Subtração)
Subtrai o número anterior pelo proximo. <br>
Também indica números negativos.

### `*` (Multiplicação)
Multiplica o número anterior pelo proximo.

### `/` (Divisão)
Divide o número anterior pelo proximo.

## Outras Funções:

### `moeda` ou `coin`
Responde com "Cara" ou "Coroa".

### `(` e `)`
Dá prioridade a calculos entre parênteses.

### `"` ou `'` (Forçar comentário)
Apartir desse comando, tudo será considerado comentário.

### `?` (Debug)
Ao colocar um `?` no início de tudo o resultado também mostra como aquele resultado foi alcançado.

# Observações:
Caso não tenha sido forçado nenhum comentário, será criado um comentário automáticamente se preciso. <br>
Será formado apartir da parte que não fizer sentido no prompt.
