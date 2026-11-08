# pawn-crypto

> Plugin de criptografia para SA-MP e Open Multiplayer, escrito em Rust — por [NullSablex](https://github.com/NullSablex)

[English](README.md) | Português (Brasil)

[![Rust CI](https://github.com/NullSablex/pawn-crypto/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/rust.yml)
[![Arquivos Pawn](https://github.com/NullSablex/pawn-crypto/actions/workflows/encoding.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/encoding.yml)
[![Auditoria de dependências](https://github.com/NullSablex/pawn-crypto/actions/workflows/audit.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/audit.yml)
[![Documentação](https://github.com/NullSablex/pawn-crypto/actions/workflows/docs.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/docs.yml)
[![CodeQL](https://github.com/NullSablex/pawn-crypto/actions/workflows/codeql.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/codeql.yml)
[![DCO](https://github.com/NullSablex/pawn-crypto/actions/workflows/contributions.yml/badge.svg?event=pull_request)](DCO)
[![CLA](https://img.shields.io/badge/CLA-required-blue)](CLA.md)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/NullSablex/pawn-crypto/badge)](https://scorecard.dev/viewer/?uri=github.com/NullSablex/pawn-crypto)
[![Licença](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
![SA-MP](https://img.shields.io/badge/SA--MP-plugin-orange)
![open.mp](https://img.shields.io/badge/open.mp-native%20%26%20legacy-orange)
![Rust](https://img.shields.io/badge/Rust-2024%20edition-orange)
![Plataformas](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows-green)
![Arquitetura](https://img.shields.io/badge/arch-x86%20%2832--bit%29-lightgrey)
[![Release](https://img.shields.io/github/v/release/NullSablex/pawn-crypto?label=release)](https://github.com/NullSablex/pawn-crypto/releases/latest)
[![Mais recente, incluindo pré-releases](https://img.shields.io/github/v/release/NullSablex/pawn-crypto?include_prereleases&label=latest%20%28incl.%20pre%29)](https://github.com/NullSablex/pawn-crypto/releases)
[![Downloads](https://img.shields.io/github/downloads/NullSablex/pawn-crypto/total)](https://github.com/NullSablex/pawn-crypto/releases)
[![Estrelas](https://img.shields.io/github/stars/NullSablex/pawn-crypto)](https://github.com/NullSablex/pawn-crypto/stargazers)
[![Issues](https://img.shields.io/github/issues/NullSablex/pawn-crypto)](https://github.com/NullSablex/pawn-crypto/issues)
[![Último commit](https://img.shields.io/github/last-commit/NullSablex/pawn-crypto/master)](https://github.com/NullSablex/pawn-crypto/commits/master)

## Visão geral

O **pawn-crypto** oferece hashes SHA-2, autenticação HMAC e hashing assíncrono de
senhas com Argon2id para SA-MP e Open Multiplayer. Desenvolvido com
[rust-samp](https://github.com/NullSablex/rust-samp) e RustCrypto.

O include Pawn é **`crypto.inc`**. O mesmo binário padrão disponibiliza a
interface de plugin do SA-MP e a interface de componente nativo do open.mp.

### Recursos

| Operação | API | Execução |
| --- | --- | --- |
| SHA-224, SHA-256, SHA-384, SHA-512 | `crypto_sha224` / `256` / `384` / `512` | Síncrona, com saída hexadecimal em letras minúsculas. |
| HMAC-SHA-256, HMAC-SHA-512 | `crypto_hmac_sha256` / `512` e `_verify` | Síncrona, com tags completas e verificação em tempo constante. |
| Argon2id | `crypto_argon2id_hash`, `crypto_argon2id_verify` | Workers limitados, salts aleatórios, strings PHC e callbacks Pawn. |

A API aceita strings Pawn compactadas ou não compactadas. Os bytes de mensagens,
chaves e senhas são preservados até o primeiro byte nulo, sem decodificação ou
normalização de texto. HMAC e Argon2id são operações distintas: use a
[API de senhas](docs/passwords.md) para armazenar senhas.

## Instalação

1. Baixe `pawn_crypto.so` (Linux) ou `pawn_crypto.dll` (Windows) e
   `crypto.inc` em [Releases](https://github.com/NullSablex/pawn-crypto/releases),
   ou [compile a partir do código-fonte](docs/building.md).
2. Copie `crypto.inc` para seu diretório de includes Pawn, como `pawno/include/`
   ou `qawno/include/`.
3. Carregue o binário usando um dos modos:
   - **SA-MP:** coloque-o em `plugins/` e adicione `pawn_crypto.so` no Linux ou
     `pawn_crypto` no Windows à linha `plugins` de `server.cfg`.
   - **open.mp nativo:** coloque-o em `components/`; o servidor o carrega automaticamente.
   - **open.mp legado:** coloque-o em `plugins/` e adicione `pawn_crypto` a
     `pawn.legacy_plugins` em `config.json`.

Consulte o [guia de instalação](docs/installation.md) para ver exemplos de configuração.

## Início rápido

```pawn
#include <a_samp>
#include <crypto>

public OnFilterScriptInit()
{
    new digest[CRYPTO_SHA256_LENGTH + 1];
    if (crypto_sha256("abc", digest))
    {
        printf("SHA-256: %s", digest);
    }
    return 1;
}
```

Para usar PascalCase, instale e inclua `crypto_omp.inc` no lugar de `crypto.inc`
e chame `Crypto_Sha256`. Ele oferece aliases Pawn para as mesmas natives.
Escolha um include por script; os dois estilos funcionam no SA-MP e no open.mp.

## Documentação

Leia a [documentação online](https://nullsablex.github.io/pawn-crypto/),
ou consulte a [visão geral em Markdown](docs/index.md) no GitHub. Os guias estão em inglês.

| Guia | Conteúdo |
| --- | --- |
| [Instalação](docs/installation.md) | Local dos binários e configuração do servidor. |
| [SHA-2](docs/hashing.md) | Hashes de dados e buffers de saída. |
| [HMAC](docs/hmac.md) | Autenticação de mensagens e verificação de tags. |
| [Argon2id](docs/passwords.md) | Armazenamento de senhas, callbacks e limites de recursos. |
| [Referência da API](docs/api-reference.md) | Assinaturas das natives, aliases, constantes e valores de retorno. |
| [Erros](docs/errors.md) | Falhas de submissão, resultados de callbacks e diagnóstico de problemas. |
| [Segurança](docs/security.md) | Tratamento de dados, segredos e responsabilidades da aplicação. |
| [Compilação](docs/building.md) | Toolchain, scripts de build e comandos de validação. |
| [Licenças das dependências](docs/licenses.md) | Inventários do cargo-about e avisos de terceiros. |

## Exemplos

Os [exemplos](examples/GUIDE.md) cobrem SHA-2, HMAC e Argon2id assíncrono nos dois
estilos de nomes. Cada arquivo `.pwn` é um filterscript executável.

## Compilação a partir do código-fonte

Use a toolchain Rust fixada em [rust-toolchain.toml](rust-toolchain.toml).
Os scripts de build produzem `dist/pawn_crypto.so` e `dist/pawn_crypto.dll`:

```sh
./scripts/build-linux.sh
./scripts/build-windows.sh
```

Consulte o [guia de compilação](docs/building.md) para os requisitos de cada
plataforma e a configuração de compilação cruzada.

## Contribuição

Consulte [CONTRIBUTING.md](CONTRIBUTING.md) para os requisitos de desenvolvimento
e revisão. O `Signed-off-by` de cada contribuidor certifica o [DCO](DCO) e aceita
o [CLA](CLA.md), conforme a política de contribuição. A participação segue o
[Código de Conduta](CODE_OF_CONDUCT.md) e a [política de uso de IA](AI-POLICY.md).

Relate vulnerabilidades em particular, conforme [SECURITY.md](SECURITY.md).
As alterações são registradas no [CHANGELOG.md](CHANGELOG.md).

## Licença

Copyright (c) 2026 NullSablex

Licenciado sob a [Licença Apache, Versão 2.0](LICENSE). As dependências de
terceiros mantêm suas próprias licenças e avisos; consulte o
[guia de licenças das dependências](docs/licenses.md).
