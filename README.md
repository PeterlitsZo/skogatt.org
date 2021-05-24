peterlits.com
===============================================================================

peterlits.com 是一个提供动态页面的网页，后端使用 rust，前端使用 React JS 框架，
并使用 docker 工具作为容器，nginx 作为其反向代理。

运行
-------------------------------------------------------------------------------
使用 `make` 命令来生成项目的对应镜像（`peterlitszo/peterlits.com`），在此之后，
使用 `make run` 和 `make stop` 来运行或停止 `peterlitszo/peterlits.com`。

使用 `make save` 对 `peterlitszo/peterlits.com` 镜像进行打包。
