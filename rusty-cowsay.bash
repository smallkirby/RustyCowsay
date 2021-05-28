_rusty-cowsay() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            rusty-cowsay)
                cmd="rusty-cowsay"
                ;;
            
            *)
                ;;
        esac
    done

    case "${cmd}" in
        rusty-cowsay)
            opts=" -b -d -g -p -s -t -w -y -L -N -l -h -V -T -e -f -W  --borg --dead --greedy --paranoid --stoned --tired --wired --young --list --help --version --tongue --eyes --file --wrapcols  <msg>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --tongue)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -T)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --eyes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wrapcols)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -W)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
    esac
}

complete -F _rusty-cowsay -o bashdefault -o default rusty-cowsay
