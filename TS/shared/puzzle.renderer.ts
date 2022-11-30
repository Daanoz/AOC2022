/* eslint-disable @typescript-eslint/adjacent-overload-signatures */
/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/explicit-module-boundary-types */
import { PuzzleServer } from './puzzle.server'

export class PuzzleRenderer implements CanvasRenderingContext2D {
    constructor(private server?: PuzzleServer) {}

    protected setProp(propKey: string, propValue: unknown): void {
        if (!this.server) { return }
        this.server.addToBuffer(
            'PROP',
            propKey,
            [propValue]
        )
    }
    protected callMethod(funcKey: string, args: unknown[]): void {
        if (!this.server) { return }
        this.server.addToBuffer(
            'FUNC',
            funcKey,
            args
        )
    }
    public clear(): void {
        if (!this.server) { return }
        this.server.clearCanvas()
    }
    public flush(): void {
        if (!this.server) { return }
        this.server.flush()
    }

    /**
     * Captured canvas methods
     */
    get canvas(): HTMLCanvasElement { throw new Error('Method not implemented.') }
    set canvas(_val: HTMLCanvasElement) { throw new Error('Method not implemented.') }

    private _globalAlpha = 1
    private _globalCompositeOperation: GlobalCompositeOperation = 'source-over'
    private _fillStyle: string | CanvasGradient | CanvasPattern = '#000000'
    private _strokeStyle: string | CanvasGradient | CanvasPattern = '#000000'
    private _filter = 'none'
    private _fontKerning: CanvasFontKerning = 'auto'
    private _imageSmoothingEnabled = true
    private _imageSmoothingQuality: ImageSmoothingQuality = 'low'
    private _lineCap: CanvasLineCap = 'butt'
    private _lineDashOffset = 0
    private _lineJoin: CanvasLineJoin = 'miter'
    private _lineWidth = 1
    private _miterLimit = 10
    private _shadowBlur = 0
    private _shadowColor = 'rgba(0, 0, 0, 0)'
    private _shadowOffsetX = 0
    private _shadowOffsetY = 0
    private _direction: CanvasDirection = 'ltr'
    private _font = '10px sans-serif'
    private _textAlign: CanvasTextAlign = 'start'
    private _textBaseline: CanvasTextBaseline = 'alphabetic'

    get globalAlpha(): number { return this._globalAlpha }
    get globalCompositeOperation(): GlobalCompositeOperation { return this._globalCompositeOperation }
    get fillStyle(): string | CanvasGradient | CanvasPattern { return this._fillStyle }
    get strokeStyle(): string | CanvasGradient | CanvasPattern { return this._strokeStyle }
    get filter(): string { return this._filter }
    get fontKerning(): CanvasFontKerning { return this._fontKerning }
    get imageSmoothingEnabled(): boolean { return this._imageSmoothingEnabled }
    get imageSmoothingQuality(): ImageSmoothingQuality { return this._imageSmoothingQuality }
    get lineCap(): CanvasLineCap { return this._lineCap }
    get lineDashOffset(): number { return this._lineDashOffset }
    get lineJoin(): CanvasLineJoin { return this._lineJoin }
    get lineWidth(): number { return this._lineWidth }
    get miterLimit(): number { return this._miterLimit }
    get shadowBlur(): number { return this._shadowBlur }
    get shadowColor(): string { return this._shadowColor }
    get shadowOffsetX(): number { return this._shadowOffsetX }
    get shadowOffsetY(): number { return this._shadowOffsetY }
    get direction(): CanvasDirection { return this._direction }
    get font(): string { return this._font }
    get textAlign(): CanvasTextAlign { return this._textAlign }
    get textBaseline(): CanvasTextBaseline { return this._textBaseline }

    set globalAlpha(val: number) { this._globalAlpha = val; this.setProp('globalAlpha', val) }
    set globalCompositeOperation(val: GlobalCompositeOperation) { this._globalCompositeOperation = val; this.setProp('globalCompositeOperation', val) }
    set fillStyle(val: string | CanvasGradient | CanvasPattern) { this._fillStyle = val; this.setProp('fillStyle', val) }
    set strokeStyle(val: string | CanvasGradient | CanvasPattern) { this._strokeStyle = val; this.setProp('strokeStyle', val) }
    set filter(val: string) { this._filter = val; this.setProp('filter', val) }
    set fontKerning(val: CanvasFontKerning) { this._fontKerning = val; this.setProp('fontKerning', val) }
    set imageSmoothingEnabled(val: boolean) { this._imageSmoothingEnabled = val; this.setProp('imageSmoothingEnabled', val) }
    set imageSmoothingQuality(val: ImageSmoothingQuality) { this._imageSmoothingQuality = val; this.setProp('imageSmoothingQuality', val) }
    set lineCap(val: CanvasLineCap) { this._lineCap = val; this.setProp('lineCap', val) }
    set lineDashOffset(val: number) { this._lineDashOffset = val; this.setProp('lineDashOffset', val) }
    set lineJoin(val: CanvasLineJoin) { this._lineJoin = val; this.setProp('lineJoin', val) }
    set lineWidth(val: number) { this._lineWidth = val; this.setProp('lineWidth', val) }
    set miterLimit(val: number) { this._miterLimit = val; this.setProp('miterLimit', val) }
    set shadowBlur(val: number) { this._shadowBlur = val; this.setProp('shadowBlur', val) }
    set shadowColor(val: string) { this._shadowColor = val; this.setProp('shadowColor', val) }
    set shadowOffsetX(val: number) { this._shadowOffsetX = val; this.setProp('shadowOffsetX', val) }
    set shadowOffsetY(val: number) { this._shadowOffsetY = val; this.setProp('shadowOffsetY', val) }
    set direction(val: CanvasDirection) { this._direction = val; this.setProp('direction', val) }
    set font(val: string) { this._font = val; this.setProp('font', val) }
    set textAlign(val: CanvasTextAlign) { this._textAlign = val; this.setProp('textAlign', val) }
    set textBaseline(val: CanvasTextBaseline) { this._textBaseline = val; this.setProp('textBaseline', val) }

    drawImage(image: CanvasImageSource, dx: number, dy: number): void;
    drawImage(image: CanvasImageSource, dx: number, dy: number, dw: number, dh: number): void;
    drawImage(image: CanvasImageSource, sx: number, sy: number, sw: number, sh: number, dx: number, dy: number, dw: number, dh: number): void;
    drawImage(...args: unknown[]): void { this.callMethod('beginPdrawImageath', args) }
    beginPath(): void { this.callMethod('beginPath', []) }
    clip(fillRule?: CanvasFillRule): void;
    clip(path: Path2D, fillRule?: CanvasFillRule): void;
    clip(...args: unknown[]): void { this.callMethod('clip', args) }
    fill(fillRule?: CanvasFillRule): void;
    fill(path: Path2D, fillRule?: CanvasFillRule): void;
    fill(...args: unknown[]): void { this.callMethod('fill', args) }
    isPointInPath(x: number, y: number, fillRule?: CanvasFillRule): boolean;
    isPointInPath(path: Path2D, x: number, y: number, fillRule?: CanvasFillRule): boolean;
    isPointInPath(...args: unknown[]): boolean {
        throw new Error('Method not implemented.')
    }
    isPointInStroke(x: number, y: number): boolean;
    isPointInStroke(path: Path2D, x: number, y: number): boolean;
    isPointInStroke(...args: unknown[]): boolean {
        throw new Error('Method not implemented.')
    }
    stroke(): void;
    stroke(path: Path2D): void;
    stroke(...args: unknown[]): void { this.callMethod('stroke', args) }
    createLinearGradient(...args: unknown[]): CanvasGradient {
        throw new Error('Method not implemented.')
    }
    createPattern(...args: unknown[]): CanvasPattern | null {
        throw new Error('Method not implemented.')
    }
    createRadialGradient(...args: unknown[]): CanvasGradient {
        throw new Error('Method not implemented.')
    }
    createImageData(sw: number, sh: number): ImageData;
    createImageData(imagedata: ImageData): ImageData;
    createImageData(..._args: unknown[]): ImageData {
        throw new Error('Method not implemented.')
    }
    getImageData(sx: number, sy: number, sw: number, sh: number): ImageData;
    getImageData(...args: unknown[]): ImageData {
        throw new Error('Method not implemented.')
    }
    putImageData(imagedata: ImageData, dx: number, dy: number): void;
    putImageData(imagedata: ImageData, dx: number, dy: number, dirtyX: number, dirtyY: number, dirtyWidth: number, dirtyHeight: number): void;
    putImageData(...args: unknown[]): void { this.callMethod('putImageData', args) }
    arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise?: boolean): void;
    arc(...args: unknown[]): void { this.callMethod('arc', args) }
    arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void;
    arcTo(...args: unknown[]): void { this.callMethod('arcTo', args) }
    bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;
    bezierCurveTo(...args: unknown[]): void { this.callMethod('bezierCurveTo', args) }
    closePath(): void { this.callMethod('closePath', []) }
    ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise?: boolean): void;
    ellipse(...args: unknown[]): void { this.callMethod('ellipse', args) }
    lineTo(x: number, y: number): void;
    lineTo(...args: unknown[]): void { this.callMethod('lineTo', args) }
    moveTo(x: number, y: number): void ;
    moveTo(...args: unknown[]): void { this.callMethod('moveTo', args) }
    quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;
    quadraticCurveTo(...args: unknown[]): void { this.callMethod('quadraticCurveTo', args) }
    rect(x: number, y: number, w: number, h: number): void;
    rect(...args: unknown[]): void { this.callMethod('rect', args) }
    getLineDash(): number[] {
        throw new Error('Method not implemented.')
    }
    setLineDash(segments: number[]): void;
    setLineDash(segments: Iterable<number>): void;
    setLineDash(...args: unknown[]): void { this.callMethod('setLineDash', args) }
    clearRect(x: number, y: number, w: number, h: number): void;
    clearRect(...args: unknown[]): void { this.callMethod('clearRect', args) }
    fillRect(x: number, y: number, w: number, h: number): void;
    fillRect(...args: unknown[]): void { this.callMethod('fillRect', args) }
    strokeRect(x: number, y: number, w: number, h: number): void;
    strokeRect(...args: unknown[]): void { this.callMethod('strokeRect', args) }
    restore(): void { this.callMethod('restore', []) }
    save(): void { this.callMethod('save', []) }
    fillText(text: string, x: number, y: number, maxWidth?: number): void;
    fillText(...args: unknown[]): void { this.callMethod('fillText', args) }
    measureText(text: string): TextMetrics {
        throw new Error('Method not implemented.')
    }
    strokeText(text: string, x: number, y: number, maxWidth?: number): void;
    strokeText(...args: unknown[]): void { this.callMethod('strokeText', args) }
    getTransform(): DOMMatrix {
        throw new Error('Method not implemented.')
    }
    resetTransform(): void { this.callMethod('resetTransform', []) }
    rotate(angle: number): void ;
    rotate(...args: unknown[]): void { this.callMethod('rotate', args) }
    scale(x: number, y: number): void;
    scale(...args: unknown[]): void { this.callMethod('scale', args) }
    setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void;
    setTransform(transform?: DOMMatrix2DInit): void;
    setTransform(...args: unknown[]): void { this.callMethod('setTransform', args) }
    transform(a: number, b: number, c: number, d: number, e: number, f: number): void;
    transform(...args: unknown[]): void { this.callMethod('transform', args) }
    translate(x: number, y: number): void;
    translate(...args: unknown[]): void { this.callMethod('translate', args) }
    drawFocusIfNeeded(element: Element): void;
    drawFocusIfNeeded(path: Path2D, element: Element): void;
    drawFocusIfNeeded(...args: unknown[]): void { this.callMethod('drawFocusIfNeeded', args) }
    scrollPathIntoView(): void;
    scrollPathIntoView(path: Path2D): void;
    scrollPathIntoView(...args: unknown[]): void { this.callMethod('scrollPathIntoView', args) }
    getContextAttributes(): CanvasRenderingContext2DSettings {
        throw new Error('Method not implemented.')
    }
    createConicGradient(startAngle: number, x: number, y: number): CanvasGradient {
        throw new Error('Method not implemented.')
    }
    roundRect(...args: unknown[]): void {
        this.callMethod('roundRect', args)
    }
}