{%- include "header" -%}
{# Keep a blank line #}
#----------------------------#
# Run
#----------------------------#
START_TIME=$(date +%s)
save START_TIME

NUM_THREADS={{ opt.parallel }}
save NUM_THREADS

# Add masurca to $PATH
export PATH="$(readlink -f $(which masurca) | xargs dirname):$PATH"

#----------------------------#
# Read stats of PE reads
#----------------------------#
log_info Symlink/copy input files
if [ ! -e pe.cor.fa ]; then
    ln -s {{ args.0 }} pe.cor.fa
fi
cp {{ args.1 }} environment.json

log_info Read stats of PE reads

SUM_COR=$( faops n50 -H -N 0 -S pe.cor.fa )
save SUM_COR

KMER="{{ opt.kmer }}"
save KMER
log_debug "You set kmer size of $KMER for the graph"

{% if opt.estsize == 'auto' -%}
ESTIMATED_GENOME_SIZE=$( cat environment.json | jq '.ESTIMATED_GENOME_SIZE | tonumber' )
{% else -%}
ESTIMATED_GENOME_SIZE={{ opt.estsize }}
save ESTIMATED_GENOME_SIZE
{% endif -%}
log_debug "ESTIMATED_GENOME_SIZE: $ESTIMATED_GENOME_SIZE"

#----------------------------#
# Build k-unitigs
#----------------------------#
if [ ! -e k_unitigs.fasta ]; then
log_info Creating k-unitigs

{% set kmers = opt.kmer | split(pat=" ") %}
{% for kmer in kmers -%}
    log_debug with k={{ kmer }}
{% if opt.tadpole -%}
    tadpole.sh \
        in=pe.cor.fa \
        out=k_unitigs_K{{ kmer }}.fasta \
        threads={{ opt.parallel }} \
        k={{ kmer }} \
        overwrite
{% else -%}
    create_k_unitigs_large_k -c $(({{ kmer }}-1)) -t {{ opt.parallel }} \
        -m {{ kmer }} -n $ESTIMATED_GENOME_SIZE -l {{ kmer }} -f 0.000001 \
        pe.cor.fa \
        > k_unitigs_K{{ kmer }}.fasta
{% endif -%}
{% endfor -%}

log_info Creating non-contained k-unitigs
    dazz contained \
{% for kmer in kmers -%}
        k_unitigs_K{{ kmer }}.fasta \
{% endfor -%}
        --len {{ opt.min }} --idt 0.98 --proportion 0.99999 --parallel {{ opt.parallel }} \
        -o k_unitigs.non-contained.fasta

if [ -s k_unitigs.non-contained.fasta ]; then
{% if opt.merge == "1" -%}
    log_info Merging k-unitigs
    dazz orient k_unitigs.non-contained.fasta \
        --len {{ opt.min }} --idt 0.99 --parallel {{ opt.parallel }} \
        -o k_unitigs.orient.fasta
    dazz merge k_unitigs.orient.fasta \
        --len {{ opt.min }} --idt 0.999 --parallel {{ opt.parallel }} \
        -o k_unitigs.fasta
{% else -%}
    mv k_unitigs.non-contained.fasta k_unitigs.fasta
{% endif -%}
else
    touch k_unitigs.fasta
fi

rm k_unitigs_K*.fasta

fi

#----------------------------#
# Done.
#----------------------------#
END_TIME=$(date +%s)
save END_TIME

RUNTIME=$((END_TIME-START_TIME))
save RUNTIME

log_info Done.

exit 0