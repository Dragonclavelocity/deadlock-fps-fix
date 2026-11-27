<div align="center">

<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest">
  <img src="assets/banner.svg" width="100%" alt="Deadlock FPS Fix: programa gratis para aumentar FPS en Deadlock, con presets, FOV personalizado y resolución estirada">
</a>

<h1>Deadlock FPS Fix: aumentar FPS en Deadlock, cambiar el FOV y usar resolución estirada, gratis</h1>

<p>
<i>Deadlock FPS Fix es un programa gratis y de código abierto para aumentar FPS en Deadlock y ajustar su configuración en Windows 10 y 11: aplica presets de FPS estimados para tu PC, pone un campo de visión personalizado y una resolución estirada o más baja, y borra la caché de shaders para quitar los tirones. Hecho para Deadlock de Valve, elige la <b>mejor configuración de Deadlock para tu PC</b>, maneja un <b>campo de visión (FOV) personalizado</b> (por encima de 90° es experimental) y una <b>resolución estirada o más baja</b>, y hace una copia de seguridad de cada archivo antes de tocar nada.</i>
</p>

<p>
<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest"><img src="https://img.shields.io/github/v/release/Dragonclavelocity/deadlock-fps-fix?style=flat&labelColor=0c0c0e&color=ebe2cf&label=versi%C3%B3n" alt="Última versión de Deadlock FPS Fix"></a>
<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/stargazers"><img src="https://img.shields.io/github/stars/Dragonclavelocity/deadlock-fps-fix?style=flat&labelColor=0c0c0e&color=3a3833&label=estrellas" alt="Estrellas en GitHub"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/licencia-MIT-3a3833?style=flat&labelColor=0c0c0e" alt="Licencia MIT"></a>
<img src="https://img.shields.io/badge/Windows-10%20%7C%2011-3a3833?style=flat&labelColor=0c0c0e" alt="Windows 10 y Windows 11">
<a href="https://github.com/DISCORD_INVITE"><img src="https://img.shields.io/badge/discord-unirse-3a3833?style=flat&labelColor=0c0c0e&logo=discord&logoColor=f1ece2" alt="Discord de Deadlock FPS Fix"></a>
</p>

<p>
<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest/download/deadlock-fps-fix.zip"><img src="https://img.shields.io/badge/Descargar-instalador%20Windows-ebe2cf?style=for-the-badge&labelColor=0c0c0e&logo=windows&logoColor=f1ece2" alt="Descargar Deadlock FPS Fix para Windows" height="38"></a>
<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest/download/deadlock-fps-fix.zip"><img src="https://img.shields.io/badge/Portable-.zip-3a3833?style=for-the-badge&labelColor=0c0c0e" alt="Descargar Deadlock FPS Fix portable" height="38"></a>
</p>

<p>
<a href="README.md">English</a> · <b>Español</b> · <a href="README.ar.md">العربية</a> · <a href="README.hi.md">हिन्दी</a> · <a href="README.zh-CN.md">简体中文</a>
<br>
<a href="#fotogramas">Fotogramas</a> ·
<a href="#campo-de-vision">Campo de visión</a> ·
<a href="#resolucion">Resolución</a> ·
<a href="#los-presets-lado-a-lado">Presets</a> ·
<a href="#fotos-fijas">Capturas</a> ·
<a href="#accion-descarga-y-primeros-pasos">Descarga</a> ·
<a href="#preguntas-frecuentes">Preguntas</a>
</p>

</div>

<br>

> En la primera línea, Deadlock va fluido. Cuarenta minutos después, con seis héroes, definitivas y almas por todos lados en la misma pelea, ya no tanto. Deadlock FPS Fix está pensado para ese momento. Son tres capítulos cortos: **fotogramas**, **campo de visión** y **resolución**. Nada más.

## De un vistazo

| | |
|---|---|
| **Qué es** | Una app de Windows que sube los FPS y quita los tirones en Deadlock de Valve con presets, campo de visión y resolución |
| **Precio y licencia** | Gratis, código abierto con licencia MIT. Sin cuenta, sin anuncios, sin telemetría |
| **Plataformas** | Windows 10 (21H2 o posterior) y Windows 11 de 64 bits, Deadlock instalado con Steam. Sin Linux ni Steam Deck en la 1.0 |
| **Datos** | Solo se conecta a internet para buscar actualizaciones en GitHub Releases, y puedes desactivarlo |
| **Funciones principales** | Tres presets con FPS estimados · deslizador de FOV con vista previa · resolución nativa, 16:9 o 4:3 estirada · borrar caché de shaders · opciones de lanzamiento · copias y restauración en un clic |
| **Qué cambia** | `video.txt`, `gameinfo.gi` (solo para un FOV por encima de 90°), las opciones de lanzamiento que pegas en Steam, las carpetas de caché de shaders y, en Windows, el Modo de juego, la programación de GPU y el plan de energía |
| **Qué nunca toca** | El proceso del juego ni su memoria, ejecutables, mapas y archivos del juego, tu cuenta de Steam |
| **Anticheat** | Deadlock usa VAC. No se inyecta nada ni se lee memoria. El FOV por encima de 90° es un método experimental y no oficial de la comunidad que Valve puede cambiar o bloquear; para el menor riesgo, deja el FOV en 90° o menos. No podemos prometer nada en nombre de Valve |
| **Alternativa a** | El menú de opciones del juego (FOV hasta 90°), editar archivos a mano, optimizadores de juegos genéricos |

<br>

<a name="fotogramas"></a>
<h2 align="center"><sub>CAPÍTULO I</sub><br>Fotogramas</h2>

<p align="center"><i>Tres presets. Uno de ellos es el indicado para tu PC, y la app te dice cuál.</i></p>

Deadlock FPS Fix detecta tu tarjeta gráfica, tu procesador y tu monitor, y estima los FPS de tres presets en **peleas de equipo de final de partida**, que es donde las caídas de FPS en Deadlock de verdad molestan. **Competitivo** deja la imagen limpia y plana para leer bien las peleas. **Equilibrado** se ve casi como el valor por defecto y elimina las caídas. **Calidad** mantiene todos los efectos y solo quita los tirones. Antes de aplicar uno, la app guarda una copia de seguridad, borra la caché de shaders y escribe los valores en el propio `video.txt` del juego.

<div align="center">
<img src="assets/gifs/deadlock-fps-presets.gif" width="88%" alt="Demo para aumentar FPS en Deadlock: Aplicar Equilibrado guarda una copia, borra la caché de shaders y escribe video.txt">
<br><sub><i>Aplicar Equilibrado: copia de seguridad, caché de shaders, video.txt y opciones de lanzamiento. Después, reinicia Deadlock.</i></sub>
</div>

<br>

<a name="campo-de-vision"></a>
<h2 align="center"><sub>CAPÍTULO II</sub><br>Campo de visión</h2>

<p align="center"><i>El control Camera FOV del juego llega hasta 90°. A veces quieres ver la línea entera.</i></p>

Mueve el deslizador y la vista previa te muestra lo que ve la cámara del héroe, al lado del máximo del juego. **Hasta 90°** la app usa el mismo valor que el control Camera FOV del juego. **Por encima de 90°** usa el método de la comunidad: un valor `r_aspectratio` en `game\citadel\gameinfo.gi` (unos `3.07` para 110°). Esa parte es **experimental**: no es un ajuste oficial de Valve, cada actualización del juego restaura el archivo original y la app lo vuelve a aplicar en el siguiente inicio si así lo quieres. Antes del primer cambio se guarda una copia del archivo.

<div align="center">
<img src="assets/gifs/deadlock-fov-slider.gif" width="88%" alt="Demo para cambiar el FOV en Deadlock: el deslizador va de 110 a 125 y baja a 90 grados con vista previa en vivo">
<br><sub><i>Por encima de 90°, el marco punteado muestra lo que se vería con el máximo del juego.</i></sub>
</div>

<br>

<a name="resolucion"></a>
<h2 align="center"><sub>CAPÍTULO III</sub><br>Resolución</h2>

<p align="center"><i>Menos píxeles, la misma pantalla.</i></p>

Elige **Nativa**, una resolución **16:9** más baja (por ejemplo 1920×1080 en un monitor 1440p), **4:3 estirada** (1440×1080 o 1280×960) o escribe un tamaño a medida. La app guarda la resolución en `video.txt` y añade `-w` y `-h` a tus opciones de lanzamiento. El estirado en sí lo hace el **driver de tu tarjeta gráfica**: la app revisa si el escalado por GPU está activo y te indica dónde activarlo (AMD Software: *Escalado de GPU* y *Modo de escalado: Panel completo*; Panel de control de NVIDIA: escalado a *Pantalla completa* en la GPU). La resolución estirada en Deadlock necesita el modo Pantalla completa, y algunos jugadores reportan que en 4:3 se cortan elementos del HUD en los bordes.

<div align="center">
<img src="assets/gifs/deadlock-stretched-resolution.gif" width="88%" alt="Demo de resolución estirada en Deadlock: cambio de 1920x1080 a 4:3 1440x1080 estirada y de vuelta">
<br><sub><i>FPS estimados con el preset Equilibrado en el PC de prueba: 108 en nativa, 141 en 1080p, 156 en 4:3.</i></sub>
</div>

<br>

## Los presets, lado a lado

Estimaciones en el PC de prueba (RX 6700 XT, Ryzen 5 5600, 2560×1440 a 165 Hz, peleas de final de partida, 94 FPS antes). Tus números serán distintos; la app los estima para tu hardware.

| | Competitivo | **Equilibrado** | Calidad |
|---|:---:|:---:|:---:|
| FPS estimados | **168** (+74) | **141** (+47) | **112** (+18) |
| Escalado | FSR Rendimiento | FSR Calidad | Nativo |
| Sombras | No | Bajas | Medias |
| Oclusión ambiental | No | No | Media |
| Límite de FPS | 237 | 162 | 162 |
| Aspecto | Plano y limpio | Casi como por defecto | Todos los efectos |
| Elígelo si | Juegas ranked y quieres el máximo de FPS | No quieres caídas en las peleas | Tu PC es potente y solo quieres quitar el lag y los tirones |

Los tres borran la caché de shaders y guardan una copia antes. El campo de visión y la resolución van aparte y funcionan con cualquier preset.

<a name="fotos-fijas"></a>
## Fotos fijas

<table>
<tr>
<td width="33%"><img src="assets/screenshots/deadlock-fps-presets.png" alt="Pantalla de presets para aumentar FPS en Deadlock: Competitivo, Equilibrado y Calidad con FPS estimados"><br><sub><b>Presets de FPS.</b> FPS estimados para tu PC.</sub></td>
<td width="33%"><img src="assets/screenshots/deadlock-fov-changer.png" alt="Pantalla para cambiar el FOV en Deadlock comparando 90 y 110 grados"><br><sub><b>Campo de visión.</b> 90° junto a tu valor.</sub></td>
<td width="33%"><img src="assets/screenshots/deadlock-stretched-resolution.png" alt="Pantalla de resolución estirada en Deadlock: nativa, 16:9 1080p y 4:3 1440x1080"><br><sub><b>Resolución.</b> Nativa, 16:9 o 4:3 estirada.</sub></td>
</tr>
<tr>
<td width="33%"><img src="assets/screenshots/deadlock-launch-options.png" alt="Opciones de lanzamiento de Deadlock: -novid, -w -h y +fps_max, y botón para borrar la caché de shaders"><br><sub><b>Opciones de lanzamiento.</b> Y una caché de shaders limpia.</sub></td>
<td width="33%"><img src="assets/screenshots/deadlock-settings-backup-restore.png" alt="Copias de seguridad de la configuración de Deadlock con historial y tabla de ahora frente a la copia"><br><sub><b>Copias.</b> Cada cambio, a un clic de deshacerse.</sub></td>
<td width="33%"><img src="assets/screenshots/deadlock-fps-fix-settings.png" alt="Ajustes de Deadlock FPS Fix con la carpeta del juego, la ruta de video.txt y lo que cambia la app"><br><sub><b>Ajustes.</b> Lo que cambia y lo que nunca hace.</sub></td>
</tr>
</table>

## Lo que cambia y lo que nunca toca

| Cambia | Nunca toca |
|---|---|
| `game\citadel\cfg\video.txt` (gráficos y resolución) | El proceso del juego ni su memoria |
| `game\citadel\gameinfo.gi`, solo para un FOV por encima de 90° | Ejecutables, mapas u otros archivos del juego |
| Opciones de lanzamiento, que tú mismo pegas en Steam | El anticheat (Deadlock usa VAC) |
| Carpetas de caché de shaders (precarga de Steam y caché de DirectX) | Tu cuenta de Steam ni los archivos de Steam |
| Windows: Modo de juego, programación de GPU acelerada por hardware, plan de energía | Nada sin hacer una copia antes |

No se inyecta nada: no hay overlay, ni driver, ni servicio en segundo plano. Todo lo que hace la app lo podrías hacer a mano con un editor de texto y la configuración de Windows; la app solo lo hace más rápido y guarda copias.

<a name="accion-descarga-y-primeros-pasos"></a>
## Acción: descarga y primeros pasos

| Paquete | Archivo | Notas |
|---|---|---|
| Instalador | [`DeadlockFPSFix-Setup-1.0.0-x64.exe`](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest/download/deadlock-fps-fix.zip) | Acceso en el menú Inicio, se actualiza desde GitHub Releases |
| Portable | [`DeadlockFPSFix-1.0.0-portable-x64.zip`](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest/download/deadlock-fps-fix.zip) | Descomprime y ejecuta, la configuración queda en la carpeta |
| winget | `winget install DeadlockFPSFix.DeadlockFPSFix` | Cuando el paquete sea aceptado |
| Scoop | `scoop install extras/deadlock-fps-fix` | Cuando el paquete sea aceptado |

**Requisitos:** Windows 10 (21H2 o posterior) o Windows 11 de 64 bits y Deadlock instalado con Steam. Ocupa unos 15 MB.

1. **Cierra Deadlock** y abre Deadlock FPS Fix. Encuentra solo tu biblioteca de Steam y el archivo `video.txt`.
2. **Aplica el preset** marcado como *Ideal para tu PC*. Si quieres, ajusta después el campo de visión y la resolución.
3. **Pega las opciones de lanzamiento** en Steam (Biblioteca → clic derecho en Deadlock → Propiedades → General → Opciones de lanzamiento) y entra a una partida.

Descarga solo desde [GitHub Releases](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases), con sumas SHA-256.

## Otras formas de hacerlo

Según lo que vimos al momento de escribir esto; revisa cada opción tú mismo, porque cambian.

| | Deadlock FPS Fix | Menú de opciones del juego | Editar archivos a mano | Optimizadores de juegos genéricos |
|---|:---:|:---:|:---:|:---:|
| Precio | Gratis, MIT | Gratis | Gratis | A menudo gratis con planes de pago |
| Presets pensados para Deadlock | ✅ | ❌ | ❌ | Normalmente ❌ |
| FOV por encima de 90° | ✅ experimental | ❌ (máx. 90°) | ✅ si conoces el archivo | ❌ |
| Guía de resolución estirada | ✅ | En parte | ✅ | ❌ |
| Borrar caché de shaders | ✅ | ❌ | ✅ a mano | A veces |
| Copia antes de cada cambio | ✅ | ❌ | Solo si la haces tú | Depende |
| Código abierto | ✅ | n/a | n/a | Normalmente ❌ |

Si buscabas una alternativa gratis a los optimizadores genéricos que conozca los archivos propios de Deadlock, es esta.

## Escenas

**Aumentar FPS en Deadlock con un PC de bajos recursos.** Empieza con Competitivo y luego prueba 16:9 a 1600×900. Borrar la caché de shaders después de un parche grande muchas veces quita por sí solo los tirones de la primera partida.

**Quitar los tirones de Deadlock después de una actualización.** Las actualizaciones traen shaders nuevos y a veces restauran archivos. Borra la caché una vez, juega una partida para que se reconstruya y revisa la pestaña Copias: la app anota cuándo una actualización restauró `gameinfo.gi` y vuelve a poner tu campo de visión.

**La mejor configuración de Deadlock para ranked.** Competitivo con el límite de FPS en tu frecuencia de refresco menos 3, o 237 en un monitor de 240 Hz. Desactivar las sombras ayuda a ver mejor, no solo a ganar FPS.

**Deadlock FOV por encima de 90.** Pestaña Campo de visión, entre 100 y 115°. Antes, lee la nota sobre el método de la comunidad.

**Resolución estirada 4:3 en Deadlock.** Pestaña Resolución, 4:3, y luego activa el escalado a panel completo en el driver. Pon Deadlock en Pantalla completa.

<a name="preguntas-frecuentes"></a>
## Preguntas frecuentes

<details><summary><b>¿Cómo cambio el FOV en Deadlock por encima de 90?</b></summary>

Solo con un método no oficial de la comunidad, porque el control Camera FOV del juego llega hasta 90°. Ese método es un valor `r_aspectratio` en el bloque `ConVars` de `game\citadel\gameinfo.gi`; Deadlock FPS Fix lo escribe por ti a partir de los grados que elijas, con copia de seguridad. Es experimental, Valve puede cambiarlo o bloquearlo, y cada actualización del juego restaura el archivo original (la app puede volver a aplicarlo al iniciar). Si prefieres quedarte solo con ajustes oficiales, deja el deslizador en 90° o menos.
</details>

<details><summary><b>¿Cómo pongo resolución estirada en Deadlock?</b></summary>

Elige una resolución 4:3 como 1440×1080 o 1280×960 (la app la escribe en `video.txt` y en las opciones `-w`/`-h`), pon Deadlock en Pantalla completa y activa el escalado a pantalla completa por GPU en AMD Software o en el Panel de control de NVIDIA. Sin escalado por GPU verás barras negras en vez de la imagen estirada. Si una resolución no aparece en la lista, créala primero como resolución personalizada en el driver.
</details>

<details><summary><b>¿Me pueden banear por usar Deadlock FPS Fix?</b></summary>

No podemos prometer nada en nombre de Valve, pero la app solo cambia archivos de configuración, te da opciones de lanzamiento y modifica algunas opciones de Windows. No toca el proceso del juego, no lee su memoria y no inyecta nada, que es lo que busca VAC. El FOV por encima de 90° edita un archivo de configuración de texto en la carpeta del juego: es un truco experimental de la comunidad, no un ajuste oficial. Para el menor riesgo, usa solo los presets y deja el FOV en 90° o menos.
</details>

<details><summary><b>¿Cuál es la mejor configuración de Deadlock para tener más FPS?</b></summary>

Para la mayoría de los PC: escalado FSR (Calidad en tarjetas de gama media, Rendimiento en las más viejas), sombras bajas o desactivadas, oclusión ambiental desactivada, V-Sync desactivado y límite de FPS en tu frecuencia de refresco menos 3. La app elige el preset para tu hardware y te muestra la estimación antes de aplicarlo.
</details>

<details><summary><b>¿Por qué Deadlock da tirones? ¿Sirve borrar la caché de shaders?</b></summary>

Sobre todo porque los shaders se compilan mientras juegas, en especial después de una actualización o de cambiar de driver, y sí, borrar la caché de shaders suele ayudar. Al borrar la precarga de shaders de Steam y la caché de DirectX, el juego los vuelve a generar; la primera partida puede dar algún tirón y después suele ir más fluido que antes. No arregla el lag del servidor ni un PC que se sobrecalienta.
</details>

<details><summary><b>¿Dónde está el archivo video.txt de Deadlock?</b></summary>

`<biblioteca de Steam>\steamapps\common\Deadlock\game\citadel\cfg\video.txt`. La app lo encuentra sola y muestra la ruta en Ajustes.
</details>

<details><summary><b>¿Qué opciones de lanzamiento uso en Deadlock?</b></summary>

`-novid` para saltar el video de inicio, `-w` y `-h` si juegas con una resolución más baja o estirada, y `+fps_max` para limitar los FPS. `-high` casi nunca ayuda. `-vulkan` cambia el renderizador; pruébalo solo si DirectX 11 te da tirones, y compara.
</details>

<details><summary><b>¿Cómo deshago todo?</b></summary>

Abre Copias, elige *Configuración original* y pulsa Restaurar. Se guardó la primera vez que abriste la app, antes de cualquier cambio. Las opciones de lanzamiento las quitas tú en Steam.
</details>

<details><summary><b>¿Es gratis? ¿Recoge datos?</b></summary>

Sí, es gratis y de código abierto con licencia MIT, y no recoge datos: sin cuenta, sin anuncios y sin telemetría. Solo se conecta a internet para buscar actualizaciones en GitHub Releases, y puedes desactivarlo.
</details>

<details><summary><b>¿Funciona en Linux o en Steam Deck?</b></summary>

En la versión 1.0, no: es una app para Windows. El soporte para Linux (prefijos de Proton) está en la hoja de ruta.
</details>

## Hoja de ruta

- [x] Tres presets con FPS estimados para tu hardware
- [x] Deslizador de campo de visión con vista previa (rango del juego y experimental por encima de 90°)
- [x] Resolución nativa, 16:9 y 4:3 estirada, tamaños a medida
- [x] Borrar caché de shaders, opciones de lanzamiento, copias y restauración
- [x] Interfaz en inglés y español
- [ ] Benchmark corto dentro del juego (tiempos de fotograma con PresentMon)
- [ ] Perfiles de campo de visión por héroe
- [ ] Soporte para Linux y Steam Deck (Proton)
- [ ] Más idiomas de la interfaz (portugués, árabe, hindi, chino)

## Hecho con

<p>
<img src="https://img.shields.io/badge/Rust-n%C3%BAcleo-3a3833?style=flat&labelColor=0c0c0e&logo=rust&logoColor=f1ece2" alt="Rust">
<img src="https://img.shields.io/badge/Tauri-2-3a3833?style=flat&labelColor=0c0c0e&logo=tauri&logoColor=f1ece2" alt="Tauri">
<img src="https://img.shields.io/badge/TypeScript-interfaz-3a3833?style=flat&labelColor=0c0c0e&logo=typescript&logoColor=f1ece2" alt="TypeScript">
</p>

## Contribuir

Los reportes de errores, los presets nuevos y las traducciones son bienvenidos. Lee primero [CONTRIBUTING.md](CONTRIBUTING.md) (en inglés): cada cambio debe poder deshacerse y limitarse a archivos de configuración, opciones de lanzamiento u opciones de Windows. Los problemas de seguridad se reportan como indica [SECURITY.md](SECURITY.md).

<a href="https://star-history.com/#Dragonclavelocity/deadlock-fps-fix&Date"><img src="https://api.star-history.com/svg?repos=Dragonclavelocity/deadlock-fps-fix&type=Date&theme=dark" width="600" alt="Historial de estrellas de Deadlock FPS Fix"></a>

<a href="https://github.com/Dragonclavelocity/deadlock-fps-fix/graphs/contributors"><img src="https://contrib.rocks/image?repo=Dragonclavelocity/deadlock-fps-fix" alt="Personas que contribuyen a Deadlock FPS Fix"></a>

## Licencia y aviso legal

MIT, ver [LICENSE](LICENSE).

Deadlock FPS Fix es un proyecto independiente hecho por fans. **No está afiliado, respaldado ni patrocinado por Valve Corporation**. Deadlock, Steam y VAC son marcas comerciales de Valve Corporation. Las capturas del juego de este repositorio son imágenes promocionales de Valve tomadas de la página de Deadlock en Steam y se usan solo como ilustración. Los FPS son estimaciones y dependen de tu hardware, tus drivers y la partida.

<div align="center">
<br>
<p>Si Deadlock FPS Fix te dio una partida más fluida, <b>⭐ dale una estrella al repo</b> para que otros jugadores lo encuentren.</p>
<sub>Palabras clave: aumentar fps deadlock, deadlock fps fix, mejor configuración deadlock, deadlock fov, cambiar fov deadlock, resolución estirada deadlock, deadlock 4:3, quitar lag deadlock, quitar tirones deadlock, caché de shaders deadlock, opciones de lanzamiento deadlock</sub>
</div>
