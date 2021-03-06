{%- include "header" -%}
{# Keep a blank line #}
#----------------------------#
# Run
#----------------------------#
log_warn 4_anchors.sh

#----------------------------#
# set parameters
#----------------------------#
USAGE="Usage: $0 [DIR_PREFIX]"

DIR_PREFIX=${1:-"4_unitigs_superreads"}

{% set parallel2 = opt.parallel | int / 2 -%}
{% set parallel2 = parallel2 | round(method="floor") -%}
{% if parallel2 < 2 %}{% set parallel2 = 2 %}{% endif -%}
parallel --no-run-if-empty --linebuffer -k -j 2 "
    if [ ! -e 4_down_sampling/Q{1}L{2}X{3}P{4}/pe.cor.fa ]; then
        exit;
    fi

    echo >&2 '==> ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}'
    if [ -e ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/anchor/anchor.fasta ]; then
        echo >&2 '    anchor.fasta already presents'
        exit;
    fi

    if [ ! -s ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/unitigs.fasta ]; then
        echo >&2 '    unitigs.fasta does not exist or is empty'
        exit;
    fi

    if [ -d ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/anchor ]; then
        rm -fr ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/anchor
    fi
    mkdir -p ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/anchor
    cd ${DIR_PREFIX}/Q{1}L{2}X{3}P{4}/anchor

    anchr anchors \
        ../unitigs.fasta \
        ../pe.cor.fa \
        --readl {{ opt.readl }} \
        --uscale {{ opt.uscale }} \
        --lscale {{ opt.lscale }} \
        -p {{ parallel2 }} \
        -o anchors.sh
    bash anchors.sh

    echo >&2
    " ::: 0 {{ opt.qual }} ::: 0 {{ opt.len }} ::: {{ opt.cov }} ::: $(printf "%03d " {0..{{ opt.splitp }}})
