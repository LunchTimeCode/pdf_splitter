

run:
    cd splitter && cargo run


convert_pdf_to_markdown:
    #!/usr/bin/env bash
    # Check if pdftotext is available
    if ! command -v pdftotext &> /dev/null; then
        echo "Error: pdftotext is not installed. Please install poppler-utils:"
        echo "  Ubuntu/Debian: sudo apt-get install poppler-utils"
        echo "  macOS: brew install poppler"
        echo "  Arch: sudo pacman -S poppler"
        exit 1
    fi

    # Create output directory if it doesn't exist
    mkdir -p out_md

    # Check if there are any PDF files
    if ! ls out_pdf/*.pdf 1> /dev/null 2>&1; then
        echo "No PDF files found in out_pdf/ directory."
        echo "Run 'just run' first to generate PDF files."
        exit 1
    fi

    echo "Converting PDF files to markdown..."
    converted=0

    for pdf in out_pdf/*.pdf; do
        if [ -f "$pdf" ]; then
            basename=$(basename "$pdf" .pdf)
            echo "Processing: $pdf"

            # Extract text and add basic markdown formatting
            {
                echo "# ${basename//_/ }"
                echo ""
                pdftotext "$pdf" -
            } > "out_md/${basename}.md"

            if [ $? -eq 0 ]; then
                echo "✓ Converted to out_md/${basename}.md"
                ((converted++))
            else
                echo "✗ Failed to convert $pdf"
            fi
        fi
    done

    echo ""
    echo "Conversion complete: $converted files processed"


convert_markdown_to_pdf:
    #!/usr/bin/env bash
    # Create output directory if it doesn't exist
    mkdir -p out_pdf

    # Check if there are any markdown files in answers directory
    if ! ls answers/*.md 1> /dev/null 2>&1; then
        echo "No markdown files found in answers/ directory."
        exit 1
    fi

    echo "Converting markdown files from answers/ to PDF..."
    converted=0

    for md in answers/*.md; do
        if [ -f "$md" ]; then
            basename=$(basename "$md" .md)
            # Remove the extra .md extension if it exists (for .md.md files)
            basename=${basename%.md}
            echo "Processing: $md"

            success=false

    
            echo Method 2: Try pandoc with different engines
            if [ "$success" = false ] && command -v pandoc &> /dev/null; then
                if pandoc "$md" -o "out_pdf/${basename}.pdf" --pdf-engine=wkhtmltopdf 2>/dev/null; then
                    success=true
                elif pandoc "$md" -o "out_pdf/${basename}.pdf" --pdf-engine=weasyprint 2>/dev/null; then
                    success=true
                elif pandoc "$md" -o "out_pdf/${basename}.pdf" 2>/dev/null; then
                    success=true
                fi
            fi

            # Method 3: Generate simple text-based PDF using enscript + ps2pdf
            if [ "$success" = false ] && command -v enscript &> /dev/null && command -v ps2pdf &> /dev/null; then
                temp_ps="/tmp/${basename}.ps"
                if enscript -p "$temp_ps" "$md" 2>/dev/null && ps2pdf "$temp_ps" "out_pdf/${basename}.pdf" 2>/dev/null; then
                    success=true
                fi
                rm -f "$temp_ps"
            fi

            if [ "$success" = true ]; then
                echo "✓ Converted to out_pdf/${basename}.pdf"
                ((converted++))
            else
                echo "✗ Failed to convert $md - try installing: wkhtmltopdf, pandoc, or enscript+ghostscript"
            fi
        fi
    done

    echo ""
    echo "Conversion complete: $converted files processed"
    if [ $converted -eq 0 ]; then
        echo ""
        echo "Installation suggestions:"
        echo "  Ubuntu/Debian: sudo apt-get install wkhtmltopdf"
        echo "  macOS: brew install wkhtmltopdf"
        echo "  Arch: sudo pacman -S wkhtmltopdf"
    fi
